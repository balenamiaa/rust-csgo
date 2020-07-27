use once_cell::sync::OnceCell;

use crate::structs::{Entity, EntityIndex, Ray, Trace, TraceContent, TraceFilterTrait, TraceType};
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[has_vtable]
#[derive(VTable, Debug)]
pub struct TraceEngine {
    pub vtable: usize,
}

impl TraceEngine {
    #[virtual_index(5)]
    pub fn trace_ray<T: TraceFilterTrait>(
        &self,
        ray: &Ray,
        mask: TraceContent,
        filter: &T,
        trace: &Trace,
    ) {
    }

    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get TraceEngine without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}
