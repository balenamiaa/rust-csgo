use once_cell::sync::OnceCell;

use crate::structs::{EntityModel, StudioHdr};
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[has_vtable]
#[derive(VTable, Debug)]
pub struct ModelInfo {
    pub vtable: usize,
}

impl ModelInfo {
    #[virtual_index(32)]
    fn __studio_model(&self, model: *const EntityModel) -> *mut StudioHdr {}

    #[virtual_index(32)]
    pub fn studio_model(&self, model: &'static EntityModel) -> Option<&'static mut StudioHdr> {
        let ptr = self.__studio_model(model);
        if ptr.is_null() {
            None
        } else {
            Some(ptr as &'static mut StudioHdr)
        }
    }

    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get ModelInfo without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}
