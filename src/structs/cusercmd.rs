use crate::maths::Vector;
use crate::structs::CmdButton;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CUserCMD {
    padding: [u8; 4],
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angles: Vector,
    pub aim_direction: Vector,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub buttons: CmdButton,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub predicted: bool,
}
