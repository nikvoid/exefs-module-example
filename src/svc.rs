//! Syscall wrappers

use core::{mem::size_of, arch::asm};

/// Formatted log using syscall 0x26
#[macro_export]
macro_rules! svc_log {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let mut buf = heapless::String::<16384>::new();
        core::write!(buf, $($tt)*).ok();
        $crate::svc::output_debug_string(&buf);
    }};
}

pub fn output_debug_string(s: &str) -> u64 {
    let ptr = s.as_ptr();
    let len = s.len();
    let mut ret;
    unsafe {
        asm!(
            "svc 0x27",
            in("x0") ptr,
            in("x1") len,
            lateout("x0") ret
        )
    }
    ret
}

pub fn brk<T>(reason: u64, info: &T) -> ! where T: Sized {
    unsafe {
        asm!(
            "svc 0x26",
            in("x0") reason,
            in("x1") info as *const T,
            in("x2") size_of::<T>(),
            options(noreturn)
        )
    }
}
