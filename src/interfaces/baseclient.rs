use once_cell::sync::OnceCell;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

use crate::structs::ClientClass;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[has_vtable]
#[derive(VTable, Debug)]
pub struct BaseClient {
    pub vtable: usize,
}

impl BaseClient {
    #[virtual_index(8)]
    pub fn get_client_class_tail(&self) -> &'static mut ClientClass {}

    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get BaseClient without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}

pub struct ClientClassIterator {
    current: &'static mut ClientClass,
}

impl Default for ClientClassIterator {
    fn default() -> Self {
        Self {
            current: BaseClient::get().get_client_class_tail(),
        }
    }
}

impl Iterator for ClientClassIterator {
    type Item = &'static mut ClientClass;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current.next;

        if next.is_null() {
            None
        } else {
            self.current = unsafe { std::mem::transmute(next) };

            Some(unsafe { std::mem::transmute(next) })
        }
    }
}
