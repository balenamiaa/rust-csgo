use crate::maths::Vector;
use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct BoneFlags: usize {
        const BONE_CALCULATE_MASK        = 0x1f;
        const BONE_PHYSICALLY_SIMULATED  = 0x01;
        const BONE_PHYSICS_PROCEDURAL    = 0x02;
        const BONE_ALWAYS_PROCEDURAL     = 0x04;
        const BONE_SCREEN_ALIGN_SPHERE   = 0x08;
        const BONE_SCREEN_ALIGN_CYLINDER = 0x10;
        const BONE_USED_MASK             = 0x0007ff00;
        const BONE_USED_BY_ANYTHING      = 0x0007ff00;
        const BONE_USED_BY_HITBOX        = 0x00000100;
        const BONE_USED_BY_ATTACHMENT    = 0x00000200;
        const BONE_USED_BY_VERTEX_mask   = 0x0003fc00;
        const BONE_USED_BY_VERTEX_LOD0   = 0x00000400;
        const BONE_USED_BY_VERTEX_LOD1   = 0x00000800;
        const BONE_USED_BY_VERTEX_LOD2   = 0x00001000;
        const BONE_USED_BY_VERTEX_LOD3   = 0x00002000;
        const BONE_USED_BY_VERTEX_LOD4   = 0x00004000;
        const BONE_USED_BY_VERTEX_LOD5   = 0x00008000;
        const BONE_USED_BY_VERTEX_LOD6   = 0x00010000;
        const BONE_USED_BY_VERTEX_LOD7   = 0x00020000;
        const BONE_USED_BY_BONE_MERGE    = 0x00040000;
        const BONE_TYPE_MASK             = 0x00f00000;
        const BONE_FIXED_ALIGNMENT       = 0x00100000;
        const BONE_HAS_SAVEFRAME_POS     = 0x00200000;
        const BONE_HAS_SAVEFRAME_ROT     = 0x00400000;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HitGroup {
    Generic,
    Head,
    Chest,
    Stomach,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    Gear = 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ModType {
    Bad0,
    Brush,
    Sprite,
    Studio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum HitBoxes {
    Head,
    Neck,
    Pelvis,
    Stomach,
    LowerChest,
    Chest,
    UpperChest,
    RightThigh,
    LeftThigh,
    RightCalf,
    LeftCalf,
    RightFoot,
    LeftFoot,
    RightHand,
    LeftHand,
    RightUpperArm,
    RightForeArm,
    LeftUpperArm,
    LeftForeArm,
    Max,
}

#[repr(C)]
pub struct StudioBone {
    name_index: usize,
    parent: i32,
    bone_controller: [i32; 6],
    pos: Vector,
    quat: [f32; 4],
    rotation: Vector,
    pos_scale: Vector,
    rot_scale: Vector,
    pose_to_bone: [[f32; 4]; 3],
    quat_alignment: [f32; 4],
    flags: usize,
    proc_type: usize,
    proc_index: usize,
    physics_bone: usize,
    surface_prop_idx: usize,
    contents: i32,
    surf_prop_lookup: i32,
    __junk_0: [u8; 28],
}

#[repr(C)]
pub struct StudioBox {
    pub bone: usize,
    group: usize,
    pub mins: Vector,
    pub maxs: Vector,
    name_idx: usize,
    __pad_0: [u8; 12],
    radius: f32,
    __junk_0: [u8; 16],
}

#[repr(C)]
pub struct StudioBoxSet {
    name_idx: usize,
    hitbox_count: usize,
    hitbox_index: usize,
}

impl StudioBoxSet {
    pub fn name(&self) -> Option<&str> {
        use std::ffi::CStr;
        unsafe {
            CStr::from_ptr((self as *const _ as *const i8).add(self.name_idx))
                .to_str()
                .ok()
        }
    }

    pub fn hitbox(&self, idx: HitBoxes) -> Option<&'static mut StudioBox> {
        let ptr = unsafe {
            ((self as *const _ as *const i8).add(self.hitbox_index) as *const StudioBox)
                .add(idx as usize)
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::mem::transmute(ptr) })
        }
    }
}

#[repr(C)]
pub struct StudioHdr {
    id: usize,
    version: i32,
    checksum: usize,
    name: [u8; 64],
    length: usize,
    eye_pos: Vector,
    illium_pos: Vector,
    hull_mins: Vector,
    hull_maxs: Vector,
    mins: Vector,
    maxs: Vector,
    flags: usize,
    bones_count: usize,
    bone_idx: usize,
    bone_controllers_count: usize,
    bone_controllers_idx: usize,
    hitbox_sets_count: usize,
    hitbox_sets_idx: usize,
    local_anim_count: usize,
    local_anim_idx: usize,
    local_seq_count: usize,
    local_seq_idx: usize,
    activity_list_version: usize,
    events_indexed: usize,
    texture_count: usize,
    texture_index: usize,
}

impl StudioHdr {
    pub fn hitbox_set(&self, idx: usize) -> Option<&'static mut StudioBoxSet> {
        if idx > self.hitbox_sets_count {
            None
        } else {
            Some(unsafe {
                std::mem::transmute(
                    ((self as *const _ as *const i8).add(self.hitbox_sets_idx)
                        as *const StudioBoxSet)
                        .add(idx),
                )
            })
        }
    }

    pub fn bone(&self, idx: usize) -> Option<&'static mut StudioBone> {
        if idx > self.bones_count {
            None
        } else {
            Some(unsafe {
                std::mem::transmute(
                    ((self as *const _ as *const i8).add(self.bone_idx) as *const Self).add(idx),
                )
            })
        }
    }
}
