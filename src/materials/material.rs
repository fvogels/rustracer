use crate::{imaging::color::Color, primitives::LocalPosition};


pub trait Material {
    fn at(&self, local_position: LocalPosition) -> MaterialProperties;
}

#[derive(Clone)]
pub struct MaterialProperties {
    pub diffuse: Color,
}
