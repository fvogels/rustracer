use crate::{imaging::color::Color, primitives::primitive::LocalPosition};

pub trait Material {
    fn at(&self, position: LocalPosition) -> MaterialProperties;
}

#[derive(Debug, Copy, Clone)]
pub struct MaterialProperties {
    pub diffuse: Color,
}
