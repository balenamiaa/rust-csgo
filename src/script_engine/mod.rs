use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

use crate::script_engine::aimbot::Aimbot;
use crate::script_engine::bhop::BHop;
use crate::structs::{CUserCMD, PanelType};
use std::any::Any;

mod aimbot;
mod bhop;
mod modulesys;

#[derive(Default)]
pub struct ScriptEngine {
    modules: Vec<Box<dyn modulesys::Module + Send>>,
}

impl ScriptEngine {
    pub fn fire_pre_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) {
        self.modules.iter_mut().for_each(|module| {
            module.pre_createmove(cmd, frame_time);
        });
    }

    pub fn fire_post_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) {
        self.modules.iter_mut().for_each(|module| {
            module.post_createmove(cmd, frame_time);
        });
    }

    pub fn fire_painttraverse(&mut self, panel_type: PanelType) {
        self.modules
            .iter_mut()
            .for_each(|module| module.paint_traverse(panel_type));
    }

    pub fn initialize(&mut self) {
        self.modules.push(Box::new(BHop::new()));
        self.modules.push(Box::new(Aimbot::new()));
    }

    pub fn global() -> MutexGuard<'static, ScriptEngine> {
        INSTANCE.lock().unwrap()
    }
}

static INSTANCE: Lazy<Mutex<ScriptEngine>> = Lazy::new(|| Mutex::new(ScriptEngine::default()));
