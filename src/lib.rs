use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn realmlib_init() {
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
        "[realmlib_ffi] add_object id={} model={} bytes={}",
        id, name, len
    );
}

#[no_mangle]
pub extern "C" fn realmlib_update_transform(
    id: i32,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
) {
    println!(
        "[realmlib_ffi] update_transform id={} pos=({}, {}, {}) rot=({}, {}, {})",
        id, x, y, z, yaw, pitch, roll
    );
}

#[no_mangle]
pub extern "C" fn realmlib_remove_object(id: i32) {
    println!("[realmlib_ffi] remove_object id={}", id);
}

#[no_mangle]
pub extern "C" fn realmlib_update_terrain(
    chunk_x: i32,
    chunk_z: i32,
    height_count: usize,
) {
    println!(
        "[realmlib_ffi] update_terrain chunk=({}, {}) points={}",
        chunk_x, chunk_z, height_count
    );
}

#[no_mangle]
pub extern "C" fn realmlib_shutdown() {
    println!("[realmlib_ffi] shutdown");
}
