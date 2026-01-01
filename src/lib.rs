use std::ffi::CStr;
use std::os::raw::{c_char};

#[no_mangle]
pub extern "C" fn realmlib_init() {
    // Later: initialize realmlib world state
    println!("[realmlib_ffi] init");
}

#[no_mangle]
pub extern "C" fn realmlib_add_object(
    id: i32,
    model_name: *const c_char,
    data: *const u8,
    len: usize,
) {
    let name = unsafe {
        CStr::from_ptr(model_name).to_string_lossy()
    };

    println!(
        "[realmlib_ffi] add object id={} model={} bytes={}",
        id, name, len
    );

    // NEXT STEP (later):
    // realmlib::world.add_object_from_memory(...)
}

#[no_mangle]
pub extern "C" fn realmlib_shutdown() {
    println!("[realmlib_ffi] shutdown");
}
