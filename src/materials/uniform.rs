use crate::{imaging::color::Color, math::Ray, util::Constant};

use super::material::Material;

pub struct UniformMaterial {
    color: Color,
}

impl UniformMaterial {
    pub fn new(color: Color) -> Self {
        UniformMaterial { color }
    }
}

impl Material for UniformMaterial {
    fn at(&self, ray: Ray, trace: Box<dyn Fn(crate::math::Ray) -> Color>) -> super::material::MaterialResult {
        Box::new(Constant::new(self.color))
    }
}
