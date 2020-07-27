#![feature(clamp)]
#![feature(abi_thiscall)]
#![feature(const_generics)]
#![feature(const_fn)]

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};

mod entry;
mod interfaces;
mod maths;
mod memory;
mod script_engine;
mod structs;
mod utilities;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID,
) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => entry::on_attach(dll_module),
        DLL_PROCESS_DETACH => entry::on_detach(),
        _ => (),
    }

    minwindef::TRUE
}
