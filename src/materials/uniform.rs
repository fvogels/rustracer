use crate::{imaging::color::Color, math::{Ray, Vector}, primitives::LocalPosition};

use super::material::{Material, MaterialProperties};

pub struct UniformMaterial {
    properties: MaterialProperties,
}

impl UniformMaterial {
    pub fn new(properties: MaterialProperties) -> Self {
        UniformMaterial { properties }
    }
}

impl Material for UniformMaterial {
    fn at(&self, local_position: LocalPosition) -> MaterialProperties {
        self.properties.clone()
    }
}
