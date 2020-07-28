use crate::structs::{CUserCMD, PanelType};

pub trait Module {
    fn post_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) -> () {}
    fn paint_traverse(&mut self, panel_type: PanelType) -> () {}
}
