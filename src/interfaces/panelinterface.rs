use once_cell::sync::OnceCell;
use std::os::raw::c_char;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[has_vtable]
#[derive(VTable, Debug)]
pub struct PanelInterface {
    pub vtable: usize,
}

impl PanelInterface {
    #[virtual_index(36)]
    pub fn get_panel_name(&self, panel_id: usize) -> *const c_char {}

    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get PanelInterface without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}
