#![no_std]
#![no_main]

mod rrt0;
mod svc;


/// Called by rtld on startup
#[no_mangle]
pub extern "C" fn init() {
    svc_log!("Hello, NX64!");
    
    let mod_obj = rrt0::get_module_object();

    for node in mod_obj.iter() {
        svc_log!("{node:#?}");
    }

}
