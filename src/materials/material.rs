use crate::{math::vector3d::Vector3D, imaging::color::Color};

pub trait Material3D {
    fn at(position: Vector3D) -> MaterialProperties;
}

pub struct MaterialProperties {
    pub color: Color,
}
