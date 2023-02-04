use std::rc::Rc;

use crate::{imaging::color::Color, primitives::LocalPosition};

use super::BRDF;


pub trait Material {
    fn at(&self, local_position: LocalPosition) -> MaterialProperties;
}

#[derive(Clone)]
pub struct MaterialProperties {
    pub diffuse: Color,
    pub reflection: Color,
    pub specular_exponent: f64,
    pub specular_color: Color,
    pub brdf: Option<Rc<dyn BRDF>>,
}
