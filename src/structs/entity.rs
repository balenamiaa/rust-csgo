use netvars::prelude::*;
use vtables::VTable;
use vtables_derive::has_vtable;
use vtables_derive::virtual_index;
use vtables_derive::VTable;

use crate::interfaces::{EntityList, ModelInfo, NetvarEngine, TraceEngine};
use crate::maths::Vector;
use crate::structs::{
    BoneFlags, ClientClass, EntityFlags, EntityTeam, HitBoxes, ItemDefIndices, Ray, Trace,
    TraceContent, TraceFilterGeneric,
};
use crate::{interfaces, memory};
use bitflags::_core::ffi::c_void;
use winapi::um::libloaderapi::GetModuleHandleA;

#[allow(conflicting_repr_hints)]
#[repr(C, u32)]
pub enum EntityLifeState {
    Alive = 0,
    Dying,
    Dead,
}

#[repr(C)]
#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityCollidable {
    pub vtable: usize,
}

#[repr(C)]
#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityNetworkable {
    pub vtable: usize,
}

#[repr(C)]
#[has_vtable]
#[derive(VTable, Debug)]
pub struct EntityAnimating {
    pub vtable: usize,
}

#[repr(C)]
pub struct EntityModel {
    handle: *const std::ffi::c_void,
    name: [std::os::raw::c_char; 260],
    load_flags: isize,
    server_count: isize,
    r#type: isize,
    flags: isize,
    vec_mins: Vector,
    vec_maxs: Vector,
    radius: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EntityHandle {
    handle: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EntityIndex {
    index: usize,
}

#[repr(C)]
#[has_vtable]
#[derive(VTable, HasNetvar, Debug)]
pub struct Entity {
    pub vtable: usize,
}

impl EntityCollidable {
    #[virtual_index(1)]
    pub fn mins(&self) -> &'static Vector {}

    #[virtual_index(2)]
    pub fn maxs(&self) -> &'static Vector {}
}

impl EntityNetworkable {
    #[virtual_index(2)]
    pub fn get_clientclass(&self) -> &'static mut ClientClass {}

    #[virtual_index(10)]
    pub fn index(&self) -> EntityIndex {}
}

impl EntityAnimating {
    #[virtual_index(13)]
    pub fn setup_bones(
        &self,
        out_matrix: *const [[f32; 4]; 3],
        max_bones: usize,
        mask: usize,
        time: f32,
    ) -> bool {
    }

    #[virtual_index(9)]
    pub fn draw_model(&self, flags: isize, alpha: u8) -> () {}

    #[virtual_index(8)]
    pub fn model(&self) -> &'static mut EntityModel {}
}

impl EntityHandle {
    pub fn new<T: Into<usize>>(handle: T) -> Self {
        Self {
            handle: handle.into(),
        }
    }
}

impl EntityIndex {
    pub fn new<T: Into<usize>>(index: T) -> Self {
        Self {
            index: index.into(),
        }
    }
}

impl From<*const std::ffi::c_void> for EntityHandle {
    fn from(handle: *const c_void) -> Self {
        Self {
            handle: handle as usize,
        }
    }
}

impl Into<*const std::ffi::c_void> for EntityHandle {
    fn into(self) -> *const c_void {
        self.handle as *const c_void
    }
}

impl From<EntityHandle> for EntityIndex {
    fn from(handle: EntityHandle) -> Self {
        Self {
            index: handle.handle & 0xFFF,
        }
    }
}

#[offset_fn(interfaces::get_offset)]
impl Entity {
    #[virtual_index(218)]
    pub fn update(&self) -> () {}

    #[virtual_index(165)]
    pub fn is_weapon(&self) -> bool {}

    #[virtual_index(155)]
    pub fn is_player(&self) -> bool {}

    #[virtual_index(3)]
    pub fn collidable(&self) -> &'static EntityCollidable {}

    #[netvar(("DT_BasePlayer", "m_iHealth"))]
    pub fn health(&self) -> i32 {}

    #[netvar(("DT_CSPlayer", "m_hActiveWeapon"))]
    pub fn active_weapon_handle(&self) -> EntityHandle {}

    pub fn active_weapon(&self) -> Option<&'static Entity> {
        From::from(EntityIndex::from(self.active_weapon_handle()))
    }

    #[netvar(("DT_CSPlayer", "m_lifeState"))]
    pub fn lifestate(&self) -> EntityLifeState {}

    pub fn alive(&self) -> bool {
        (match self.lifestate() {
            EntityLifeState::Alive => true,
            _ => false,
        }) && self.health() > 0
    }

    #[netvar(("CustomTable", "Dormancy"))]
    pub fn dormant(&self) -> bool {}

    #[netvar(("DT_CSPlayer", "m_iTeamNum"))]
    pub fn team(&self) -> EntityTeam {}

    #[netvar(("DT_BaseEntity", "m_bSpotted"))]
    pub fn spotted(&self) -> bool {}

    #[netvar(("DT_CSPlayer", "m_fFlags"))]
    pub fn flags(&self) -> EntityFlags {}

    #[netvar(("DT_BaseEntity", "m_hOwnerEntity"))]
    pub fn owner_handle(&self) -> EntityHandle {}

    #[netvar(("DT_CSPlayer", "m_flSimulationTime"))]
    pub fn simulation_time(&self) -> f32 {}

    #[netvar(("DT_BasePlayer", "m_vecOrigin"))]
    pub fn origin(&self) -> Vector {}

    #[netvar(("DT_BasePlayer", "m_aimPunchAngle"))]
    pub fn aimpunch(&self) -> Vector {}

    #[netvar(("DT_BasePlayer", "m_viewPunchAngle"))]
    pub fn viewpunch(&self) -> Vector {}

    #[netvar(("DT_CSPlayer", "m_iShotsFired"))]
    pub fn shotsfired(&self) -> usize {}

    #[netvar(("DT_BasePlayer", "m_vecViewOffset[0]"))]
    pub fn view_offset(&self) -> Vector {}

    #[netvar(("CustomTable", "InReload"))]
    pub fn is_reloading(&self) -> bool {}

    #[netvar(("DT_BaseCombatWeapon", "m_iClip1"))]
    pub fn clip1_rem(&self) -> isize {}

    #[netvar(("DT_CSPlayer", "m_nTickBase"))]
    pub fn tickbase(&self) -> isize {}

    #[netvar(("DT_BaseCombatWeapon", "m_flNextPrimaryAttack"))]
    pub fn next_primary_attack(&self) -> f32 {}

    #[netvar(("DT_BaseCombatCharacter", "m_flNextAttack"))]
    pub fn next_attack(&self) -> f32 {}

    #[netvar(("DT_BasePlayer", "m_vecVelocity[0]"))]
    pub fn vel(&self) -> Vector {}

    pub fn weapon_id(&self) -> ItemDefIndices {
        unsafe {
            *std::mem::transmute::<_, &ItemDefIndices>((self as *const _ as *const u8).add(0x2FAA))
        }
    }

    pub fn eye(&self) -> Vector {
        self.origin() + self.view_offset()
    }

    pub fn is_visible(&self, other: &Entity, position: Vector) -> bool {
        let trace: Trace = unsafe { std::mem::zeroed() };
        let ray = Ray::new(self.eye().into(), position);
        let mut filter = TraceFilterGeneric::new(self);
        TraceEngine::get().trace_ray(&ray, TraceContent::HUH, &filter, &trace);

        trace.ptr_entity as *const _ as usize == other as *const _ as usize || trace.fraction == 1e0
    }

    pub fn hitbox_center(&self, id: HitBoxes) -> Option<Vector> {
        let mut bone_matrices = [[[0f32; 4]; 3]; 128];
        if self.animating().setup_bones(
            bone_matrices.as_ptr(),
            128,
            BoneFlags::BONE_USED_BY_HITBOX.bits(),
            0f32,
        ) {
            if let Some(studio_model) = ModelInfo::get().studio_model(self.animating().model()) {
                if let Some(hitbox_set) = studio_model.hitbox_set(0) {
                    if let Some(hitbox) = hitbox_set.hitbox(id) {
                        let bone_matrix = bone_matrices[hitbox.bone];

                        fn transform(bone_matrix: &[[f32; 4]; 3], mins: Vector) -> Vector {
                            let vec_x: Vector =
                                [bone_matrix[0][0], bone_matrix[0][1], bone_matrix[0][2]].into();

                            let vec_y: Vector =
                                [bone_matrix[1][0], bone_matrix[1][1], bone_matrix[1][2]].into();

                            let vec_z: Vector =
                                [bone_matrix[2][0], bone_matrix[2][1], bone_matrix[2][2]].into();

                            Vector::new(
                                mins * vec_x + bone_matrix[0][3],
                                mins * vec_y + bone_matrix[1][3],
                                mins * vec_z + bone_matrix[2][3],
                            )
                        }

                        let mut res = transform(&bone_matrix, hitbox.mins.into())
                            + transform(&bone_matrix, hitbox.maxs.into());
                        res.as_mut().iter_mut().for_each(|x| *x /= 2e0);

                        return Some(res);
                    }
                }
            }
        }

        None
    }

    pub fn animating(&self) -> &'static EntityAnimating {
        unsafe { std::mem::transmute((self as *const _ as *const usize).offset(1)) }
    }

    pub fn networkable(&self) -> &'static EntityNetworkable {
        unsafe { std::mem::transmute((self as *const _ as *const usize).offset(2)) }
    }

    //Is this even useful?
    /// #Returns an EntityIterator that starts from the current entity
    pub fn iter(&self) -> EntityIterator {
        EntityIterator {
            count: self.networkable().index().index,
        }
    }

    pub fn local() -> Option<&'static Self> {
        match *interfaces::LOCAL_PLAYER_PTR.lock().unwrap() {
            0 => None,
            ptr => Some(unsafe { std::mem::transmute::<_, &'static Self>(ptr) }),
        }
    }
}

impl From<EntityHandle> for Option<&'static Entity> {
    fn from(handle: EntityHandle) -> Self {
        let ptr = EntityList::get().entity_ptr_from_handle(handle);

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::mem::transmute::<_, &'static Entity>(ptr) })
        }
    }
}

impl From<EntityIndex> for Option<&'static Entity> {
    fn from(index: EntityIndex) -> Self {
        let ptr = EntityList::get().entity_ptr_from_index(index);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::mem::transmute::<_, &'static Entity>(ptr) })
        }
    }
}

impl PartialEq<&'static Entity> for &'static Entity {
    fn eq(&self, other: &&'static Entity) -> bool {
        (*self as *const Entity).eq(&(*other as *const Entity))
    }
}

pub struct EntityIterator {
    count: usize,
}

impl EntityIterator {
    pub fn new() -> Self {
        Self { count: 1 }
    }
}

impl Iterator for EntityIterator {
    type Item = Option<&'static Entity>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.count <= EntityList::get().highest_entity_index() {
            Some(From::from(EntityIndex::new(self.count)))
        } else {
            None
        };

        self.count += 1;

        ret
    }
}
