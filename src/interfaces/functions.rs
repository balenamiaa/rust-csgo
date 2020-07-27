use crate::memory::get_module_handle;

pub fn create_interface(module_name: &str, interface_name: &str) -> *mut std::os::raw::c_void {
    use super::types::CreateInterfaceFn;
    use std::ffi::CString;

    unsafe {
        let cstr_interface_name = CString::new(interface_name).unwrap();

        let module = get_module_handle(module_name);
        let function_ptr = winapi::um::libloaderapi::GetProcAddress(
            module as *mut _,
            CString::new("CreateInterface").unwrap().as_ptr(),
        );

        let function: CreateInterfaceFn = std::mem::transmute::<_, CreateInterfaceFn>(function_ptr);
        let interface_ptr = function(cstr_interface_name.as_ptr(), std::ptr::null_mut())
            as *mut std::os::raw::c_void;

        if interface_ptr.is_null() {
            panic!(
                "Failed to get interface for {} in module {}",
                interface_name, module_name
            );
        }

        interface_ptr
    }
}
