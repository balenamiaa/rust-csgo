use detour::static_detour;
use std::os::raw::{c_float, c_void};
use winapi::shared::minwindef::HINSTANCE;

use winapi::um::libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};

use crate::interfaces;
use crate::interfaces::{EngineClient, EntityList, PanelInterface};
use crate::memory;
use crate::script_engine::ScriptEngine;
use crate::structs::{CUserCMD, PanelType};
use winapi::um::consoleapi::AllocConsole;

type FnCreateMove = unsafe extern "stdcall" fn(c_float, *mut CUserCMD) -> bool;
type FnPaintTraverse =
    unsafe extern "thiscall" fn(&'static PanelInterface, usize, bool, bool) -> ();

static_detour! {
  static CreateMoveHook: unsafe extern "stdcall" fn(c_float, *mut CUserCMD) -> bool;
  static PaintTraverseHook: unsafe extern "thiscall" fn(&'static PanelInterface, usize, bool, bool) -> ();
}

fn createmove_detour(input_sample_frametime: f32, cmd: *mut CUserCMD) -> bool {
    unsafe {
        {
            let local_ptr =
                EntityList::get().entity_ptr_from_index(EngineClient::get().local_player_index());
            if !local_ptr.is_null() {
                *interfaces::LOCAL_PLAYER_PTR.lock().unwrap() = local_ptr as usize;
            }
        }

        ScriptEngine::global()
            .fire_pre_createmove(std::mem::transmute(cmd), input_sample_frametime);

        let ret_value: bool = unsafe { CreateMoveHook.call(input_sample_frametime, cmd) };

        ScriptEngine::global()
            .fire_post_createmove(std::mem::transmute(cmd), input_sample_frametime);

        ret_value
    }
}

fn painttraverse_detour(
    _this: &PanelInterface,
    panel_id: usize,
    force_repaint: bool,
    allow_force: bool,
) {
    use std::hash::Hasher;

    fn hash(name: &str) -> u64 {
        let mut panel_name_hasher = fnv::FnvHasher::with_key(0x811C9DC5);
        panel_name_hasher.write(name.as_bytes());
        panel_name_hasher.finish()
    }

    let ret_value = unsafe {
        PaintTraverseHook.call(PanelInterface::get(), panel_id, force_repaint, allow_force)
    };
    let d = PanelInterface::get().get_panel_name(panel_id);
    let panel_name = unsafe {
        std::ffi::CStr::from_ptr(d)
            .to_str()
            .unwrap_or(Default::default())
    };

    if hash("MatSystemTopPanel") == hash(panel_name) {
        ScriptEngine::global().fire_painttraverse(PanelType::MatSystemTop(panel_id));
    }

    ret_value
}

pub unsafe fn on_attach(dll_module: HINSTANCE) -> () {
    DisableThreadLibraryCalls(dll_module);
    let sync_dll_module = dll_module as usize;

    std::thread::spawn(move || {
        std::panic::catch_unwind(|| {
            AllocConsole();
            interfaces::initialize();

            let clientmode = {
                let temp1 = memory::get_virtual_raw(
                    interfaces::BaseClient::get().vtable as *const c_void,
                    10,
                ) as usize;
                ((temp1 + 5) as *const *const *const c_void).read().read()
            };

            let create_move_target: FnCreateMove = std::mem::transmute(memory::get_virtual_raw(
                (clientmode as *const *const c_void).read(),
                24,
            ));
            let paint_traverse_target: FnPaintTraverse = std::mem::transmute(
                memory::get_virtual_raw(PanelInterface::get().vtable as *const c_void, 41),
            );

            CreateMoveHook
                .initialize(create_move_target, createmove_detour)
                .expect("Couldn't initialize hook for createmove")
                .enable()
                .expect("Couldn't enable hook for createmove!");
            PaintTraverseHook
                .initialize(paint_traverse_target, painttraverse_detour)
                .expect("Couldn't initialize hook for painttraverse")
                .enable()
                .expect("Couldn't enable hook for painttraverse!");

            ScriptEngine::global().initialize();
        })
        .unwrap_or_else(|_data| {
            FreeLibraryAndExitThread(sync_dll_module as HINSTANCE, 1213);
        });
    });
}

pub fn on_detach() -> () {}
