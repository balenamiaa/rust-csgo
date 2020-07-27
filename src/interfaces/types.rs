use winapi::ctypes::{c_char, c_int, c_void};

pub type CreateInterfaceFn =
    unsafe extern "cdecl" fn(name: *const c_char, return_code: *const c_int) -> *mut c_void;
