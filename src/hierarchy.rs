use std::rc::{Rc, Weak};

pub enum Shape {
    Cuboid {
        half_extents: vek::Vec3<f32>,
        offset: vek::Vec3<f32>,
    },
    Sphere {
        radius: f32,
        offset: vek::Vec3<f32>,
    },
    Cylinder {
        height: f32,
        radius: f32,
        offset: vek::Vec3<f32>,
    },
    Collection {
        primitives: Weak<Object>,
        effect: Effect,
    }
}

pub enum Effect {
    Union,
    Difference,
    SmoothUnion(f32),
    SmoothDifference(f32),
}

pub struct Object {
    pub name: String,
    pub shape: Shape,
    pub aabb: vek::Aabb<f32>,
}

pub type Hierarchy = Vec<Rc<Object>>;