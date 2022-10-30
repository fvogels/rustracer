use crate::primitives::primitive::LocalPosition;

use super::material::{Material, MaterialProperties};

pub struct UniformMaterial {
    properties: MaterialProperties,
}

impl UniformMaterial {
    pub fn new(properties: MaterialProperties) -> UniformMaterial {
        UniformMaterial { properties }
    }
}

impl Material for UniformMaterial {
    fn at(&self, _position: LocalPosition) -> MaterialProperties {
        self.properties
    }
}
