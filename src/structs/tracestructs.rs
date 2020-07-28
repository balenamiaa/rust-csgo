use crate::maths::Vector;
use crate::structs::Entity;

#[repr(C, align(16))]
#[derive(Copy, Clone, Default)]
struct VectorAligned {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl From<Vector> for VectorAligned {
    fn from(vec: Vector) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
            z: vec.z as f32,
            w: 0e0,
        }
    }
}

impl From<VectorAligned> for Vector {
    fn from(vec_aligned: VectorAligned) -> Self {
        Self::new(vec_aligned.x, vec_aligned.y, vec_aligned.z)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ray {
    start: VectorAligned,
    delta: VectorAligned,
    start_offset: VectorAligned,
    extents: VectorAligned,
    pub p_world_axis_transform_matrix: *const std::ffi::c_void,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vector, end: Vector) -> Self {
        let mut instance = unsafe { std::mem::zeroed::<Self>() };
        instance.delta = VectorAligned::from(end - start);
        instance.start = VectorAligned::from(start);
        instance.is_swept = Vector::from(instance.delta).len_sqr() != 0e0;
        instance.is_ray = true;
        instance
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSurface {
    pub name: *const i8,
    pub surface_props: i16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlane {
    pub normal: Vector,
    pub dist: f32,
    pub r#type: u8,
    pub sign_bit: u8,
    pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Trace {
    pub start: Vector,
    pub end: Vector,
    pub plane: CPlane,
    pub fraction: f32,
    pub contents: i32,
    pub disp_flags: u16,
    pub all_solid: bool,
    pub start_solid: bool,
    pub fraction_solid_left: f32,
    pub surface: CSurface,
    pub hit_group: i32,
    pub physics_bone: i16,
    pub ptr_entity: &'static Entity,
    pub hitbox: i32,
}

#[repr(C)]
pub enum TraceType {
    EVERYTHING = 0,
    WORLD_ONLY,
    ENTITIES_ONLY,
    EVERYTHING_FILTER_PROPS,
}

pub trait TraceFilterTrait {
    fn should_hit_entity(&self, entity: &Entity, contents_mask: u32) -> bool;
    fn get_trace_type(&self) -> TraceType;
}

#[repr(C)]
pub struct TraceFilterGeneric {
    vtable: usize,
    skip: *const Entity,
    vec_vtable: Vec<usize>,
}

impl TraceFilterGeneric {
    pub fn new(skip: &Entity) -> Self {
        extern "thiscall" fn should_hit_entity_wrapper(
            this: &TraceFilterGeneric,
            entity: &Entity,
            contents_mask: u32,
        ) -> bool {
            TraceFilterGeneric::should_hit_entity(this, entity, contents_mask)
        }

        extern "thiscall" fn get_trace_type_wrapper(this: &TraceFilterGeneric) -> TraceType {
            TraceFilterGeneric::get_trace_type(this)
        }

        let mut vec = Vec::<usize>::new();

        vec.push(should_hit_entity_wrapper as usize);
        vec.push(get_trace_type_wrapper as usize);

        Self {
            vtable: vec.as_ptr() as usize,
            skip,
            vec_vtable: vec,
        }
    }
}

impl TraceFilterTrait for TraceFilterGeneric {
    fn should_hit_entity(&self, entity: &Entity, contents_mask: u32) -> bool {
        entity as *const _ as usize != self.skip as *const _ as usize
    }

    fn get_trace_type(&self) -> TraceType {
        TraceType::EVERYTHING
    }
}
