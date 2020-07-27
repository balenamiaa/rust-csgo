use crate::structs::{CUserCMD, PanelType};

pub trait Module {
    fn pre_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) -> () {}
    fn post_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) -> () {}
    fn paint_traverse(&mut self, panel_type: PanelType) -> () {}
}
