use std::rc::Rc;

use super::primitive::{Hit, Primitive};
use crate::math::ray::Ray;
use crate::math::transformation3d::Transformation3D;

pub struct Transformer {
    transformation: Transformation3D,
    child: Rc<dyn Primitive>,
}

impl Transformer {
    pub fn new(transformation: Transformation3D, child: Rc<dyn Primitive>) -> Transformer {
        Transformer { transformation, child }
    }
}

impl Primitive for Transformer {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let matrix = &self.transformation.matrix;
        let inverse_matrix = &self.transformation.inverse_matrix;
        let transformed_ray = inverse_matrix * ray;
        let mut hit = self.child.find_first_positive_hit(&transformed_ray)?;

        hit.position.global = matrix * &hit.position.global;
        hit.normal = matrix * &hit.normal;

        Some(hit)
    }
}
