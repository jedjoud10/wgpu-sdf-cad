use std::num::{NonZeroU8, NonZeroU32};

#[repr(align(8))]
pub struct GpuShape {
    // type
    // 0 = cube
    // 1 = sphere 
    // 2 = cylinder
    // 3 = two objects
    _type: u8,
    
    // offset
    offset: vek::Vec3<f32>,

    // data
    // half_extents when cube
    // radius when sphere
    // height + radius when cylinder
    data: vek::Vec3<f32>,
}

pub struct GpuObject {
    // loc + rot
    matrix: vek::Mat3<f32>,
    
    // world aligned aabb
    aabb: vek::Aabb<f32>,

    // last bit (31) sets wether it is union or diff
    // discard the negative sign of f32
    effect: u32,

    shape: GpuShape,
}