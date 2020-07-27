use once_cell::sync::OnceCell;

use crate::structs::EntityIndex;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

pub static INSTANCE: OnceCell<usize> = OnceCell::new();

#[repr(C)]
pub struct Globals {
    pub real_time: f32,
    pub frame_coutn: isize,
    pub absolute_frametime: f32,
    pub absolute_frame_starttime: f32,
    pub curtime: f32,
    pub frametme: f32,
    pub max_client: usize,
    pub tick_count: usize,
    pub inv_tickspersecond: f32,
    pub interpolation_factor: f32,
    pub sim_ticks_this_frame: usize,
    pub network_protocol: isize,
    pub p_savedata: usize,
    pub is_client: bool,
    pub is_remote_client: bool,
    pub timestamp_networking_base: isize,
    pub timestamp_randomize_window: isize,
}

impl Globals {
    pub fn get() -> &'static Self {
        let ptr = INSTANCE
            .get()
            .expect("Tried to get Globals without initializing it first");

        unsafe { std::mem::transmute::<usize, &'static Self>(*ptr) }
    }
}
