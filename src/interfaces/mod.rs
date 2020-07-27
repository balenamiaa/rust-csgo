use crate::memory::get_virtual_raw;
pub use baseclient::{BaseClient, ClientClassIterator};
pub use engineclient::EngineClient;
pub use entitylist::EntityList;
pub use globals::Globals;
pub use modelinfo::ModelInfo;
pub use netvarengine::{get_offset, NetvarEngine};
use once_cell::sync::{Lazy, OnceCell};
pub use panelinterface::PanelInterface;
use std::sync::Mutex;
pub use traceengine::TraceEngine;

mod baseclient;
mod engineclient;
mod entitylist;
mod functions;
mod globals;
mod modelinfo;
mod netvarengine;
mod panelinterface;
mod traceengine;
mod types;

pub static LOCAL_PLAYER_PTR: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn initialize() {
    engineclient::INSTANCE
        .set(functions::create_interface("engine.dll", "VEngineClient014") as usize)
        .expect("Failed to create EngineClient interface");

    baseclient::INSTANCE
        .set(functions::create_interface("client.dll", "VClient018") as usize)
        .expect("Failed to create BaseClient interface");

    panelinterface::INSTANCE
        .set(functions::create_interface("vgui2.dll", "VGUI_Panel009") as usize)
        .expect("Failed to create PanelInterface interface");

    entitylist::INSTANCE
        .set(functions::create_interface("client.dll", "VClientEntityList003") as usize)
        .expect("Failed to create EntityList interface");

    traceengine::INSTANCE
        .set(functions::create_interface("engine.dll", "EngineTraceClient004") as usize)
        .expect("Failed to create TraceEngine interface");

    modelinfo::INSTANCE
        .set(functions::create_interface("engine.dll", "VModelInfoClient004") as usize)
        .expect("Failed to create ModelInfo interface");

    globals::INSTANCE
        .set(unsafe {
            ((get_virtual_raw(BaseClient::get().vtable as *const std::ffi::c_void, 11) as *const i8)
                .offset(10) as *const *const usize)
                .read()
                .read()
        })
        .expect("Failed to create Globals interface");

    netvarengine::INSTANCE
        .set(Default::default())
        .expect("Failed to create NetvarEngine interface");
}
