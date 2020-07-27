use once_cell::sync::OnceCell;

use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

use crate::structs::{EntityHandle, EntityIndex};
use std::ffi::c_void;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityList {
    pub vtable: usize,
}

impl EntityList {
    #[virtual_index(6)]
    pub fn highest_entity_index(&self) -> usize {}

    #[virtual_index(4)]
    pub fn entity_ptr_from_handle(&self, handle: EntityHandle) -> *mut c_void {}

    #[virtual_index(3)]
    pub fn entity_ptr_from_index(&self, index: EntityIndex) -> *mut c_void {}

    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get EntityList without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}
