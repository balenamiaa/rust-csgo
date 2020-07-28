use crate::structs::{CUserCMD, CmdButton, Entity, EntityFlags, EntityIterator, HitBoxes};

use super::modulesys::Module;

pub struct BHop {}

impl BHop {
    pub fn new() -> BHop {
        Self {}
    }
}

impl Module for BHop {
    fn post_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) -> () {
        if let Some(local_player) = Entity::local() {
            if !local_player.alive() {
                return;
            }

            if cmd.buttons.contains(CmdButton::IN_JUMP) {
                if local_player.flags().contains(EntityFlags::ON_GROUND) {
                    cmd.buttons.set(CmdButton::IN_JUMP, true);
                } else {
                    cmd.buttons.set(CmdButton::IN_JUMP, false);
                }
            }
        }
    }
}
