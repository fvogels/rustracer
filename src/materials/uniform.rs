use crate::{imaging::color::Color, math::{Ray, Vector}, util::Constant};

use super::material::{Material, TraceFunction};

pub struct UniformMaterial {
    color: Color,
}

impl UniformMaterial {
    pub fn new(color: Color) -> Self {
        UniformMaterial { color }
    }
}

impl Material for UniformMaterial {
    fn at(&self, direction: &Vector<3>, trace: TraceFunction) -> super::material::MaterialResult {
        Box::new(Constant::new(self.color))
    }
}
