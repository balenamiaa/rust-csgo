use once_cell::sync::OnceCell;

use crate::structs::EntityIndex;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[has_vtable]
#[derive(VTable, Debug)]
pub struct EngineClient {
    pub vtable: usize,
}

impl EngineClient {
    #[virtual_index(26)]
    pub fn in_game(&self) -> bool {}

    #[virtual_index(12)]
    pub fn local_player_index(&self) -> EntityIndex {}

    #[virtual_index(5)]
    pub fn get_screensize(&self, width: *mut i32, height: *mut i32) {}

    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get EngineClient without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}
