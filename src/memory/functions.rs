use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use winapi::{
    ctypes::c_long,
    shared::minwindef::HMODULE,
    um::winnt::{PIMAGE_DOS_HEADER, PIMAGE_NT_HEADERS},
};

fn pattern_to_bytes(pattern: String) -> Vec<i32> {
    pattern
        .replace(' ', "")
        .as_bytes()
        .chunks(2)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .into_iter()
        .filter(|&q| !q.contains('?'))
        .map(|q| i32::from_str_radix(q, 16).unwrap())
        .collect::<Vec<i32>>()
}

//https://github.com/zorftw/Ion/blob/552f99fc1f1c9e3e82e9822c19ec665185583f07/Ion/src/utils/sig.rs#L27
pub fn pattern_scan(module: HMODULE, sig: &str) -> *mut u8 {
    unsafe {
        let dos_headers = module as PIMAGE_DOS_HEADER;

        let module_addr = module as usize;
        let e_lfanew = (*dos_headers).e_lfanew as c_long;

        let nt_headers = (module_addr + e_lfanew as usize) as PIMAGE_NT_HEADERS;

        let size_of_image = (*nt_headers).OptionalHeader.SizeOfImage as usize;
        let pattern_bytes = pattern_to_bytes(sig.to_owned());
        let bytes = module as *mut u8;

        let size = pattern_bytes.len();

        for i in 0..(size_of_image - size as usize) {
            let mut found = true;
            for j in 0..size {
                if *bytes.offset(i as isize + j as isize) != pattern_bytes[j] as _
                    && pattern_bytes[j] != -1
                {
                    found = false;
                    break;
                }
            }

            if found {
                return bytes.offset(i as _) as _;
            }
        }
    }

    0 as *mut _
}

pub unsafe fn get_virtual_raw(vfmt: *const c_void, index: usize) -> *const c_void {
    (vfmt as *const *const c_void).offset(index as isize).read()
}

pub unsafe fn get_module_handle(module_name: &str) -> *const c_void {
    winapi::um::libloaderapi::GetModuleHandleA(CString::new(module_name).unwrap().as_ptr())
        as *const c_void
}
