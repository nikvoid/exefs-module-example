//! Like crt0, but r(ust)rt0.
//! This module contains bits that are essential to compile rust bare module
//! and make NX rtld pick it up

use core::arch::global_asm;
use crate::svc_log;

global_asm!( // (syntax highlight helper) c
r#"
// NOTE: .text.jmp is predefined in aarch64-nintendo-switch-freestanding target linker script
.section .text.jmp, "ax", %progbits
    .align 4
    .global __module_start
    __module_start:
        .word 0
        .word __mod0 - __module_start

    .global __mod0
    __mod0:
        .ascii "MOD0"
        .word __dynamic_start - __mod0
        .word __bss_start - __mod0
        .word __bss_end - __mod0
        .word __eh_frame_hdr_start - __mod0
        .word __eh_frame_hdr_end - __mod0
        .word __module_object - __mod0

.section .bss
    .global __module_object
    __module_object: .zero 0xB8
"#  
);

#[repr(align(0x200))]
#[repr(C)]
struct ModuleName<const N: usize> {
    _unused: u32,
    len: u32,
    name: [u8; N],
}

/// Not strictly required (at least by emulators), but looks nice
#[used]
#[no_mangle]
#[link_section = ".module_name"]
static MODULE_NAME: ModuleName<12> = ModuleName { 
    _unused: 0, 
    len: 12, 
    name: *b"exefs-module",
};

/// Global struct that filled in runtime by rtld
/// and is a statically allocated linked list node.
/// Linked list does not have start or end, it's cyclic
/// Required by rtld
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ModuleObject {
    pub next: *const ModuleObject,
    pub prev: *const ModuleObject,
    pub rela_or_rel_plt: *const u8,
    pub rela_or_rel: *const u8,
    pub module_base: *const u8,
    pub dynamic: *const u8,
    pub is_rela: bool,
    pub rela_or_rel_plt_size: usize,
    pub dt_init: unsafe extern "C" fn(),
    pub dt_fini: unsafe extern "C" fn(),
    pub hash_bucket: *const u32,
    pub hash_chain: *const u32,
    pub dynstr: *const u8,
    pub dynsym: *const u8,
    pub dynstr_size: usize,
    pub got: *const *const u8,
    pub rela_dyn_size: usize,
    pub rel_dyn_size: usize,
    pub rel_count: usize,
    pub rela_count: usize,
    pub hash_nchain_value: usize,
    pub hash_nbucket_value: usize,
    pub got_stub_ptr: *const u8
}

impl ModuleObject {
    /// Iterate through ModuleObjects of each loaded module, starting from self
    pub fn iter(&self) -> ModuleIter<'_> {
        ModuleIter { node: None, first: self }
    }
}

pub struct ModuleIter<'a> {
    first: &'a ModuleObject,
    node: Option<*const ModuleObject>,
}

impl Iterator for ModuleIter<'_> {
    type Item = ModuleObject;

    fn next(&mut self) -> Option<Self::Item> {      
        let next_node = match self.node {
            // Safe if this module was initialized by rtld
            Some(ptr) => unsafe { 
                let node = core::ptr::read_volatile(ptr);
                self.node = Some(node.next);
                node
            },
            None => {
                self.node = Some(self.first.next);
                return Some(self.first.clone());
            }
        };

        // Prevent wraparound
        if next_node.module_base != self.first.module_base {
            Some(next_node)
        } else {
            None
        }
    }
}

extern {
    #[link_name = "__module_object"]
    static MODULE_OBJECT: *const ModuleObject;
}


/// Get ModuleObject of this module
pub fn get_module_object() -> ModuleObject {
    // This extern always points to bss section, so it should be not-null
    // NOTE: Trying to use &'static references instead of volatile reads
    // certainly leads to UB in form of nullptr-derefs...
    unsafe { core::ptr::read_volatile(MODULE_OBJECT) }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    svc_log!("{info}");
    crate::svc::brk(0xEC0D, &());
}