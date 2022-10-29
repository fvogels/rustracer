use std::rc::Rc;

use crate::{materials::material::Material, math::ray::Ray};

use super::primitive::{Primitive, Hit};

pub struct Decorator {
    material: Rc<dyn Material>,
    child: Rc<dyn Primitive>,
}

impl Decorator {
    pub fn new(material: Rc<dyn Material>, child: Rc<dyn Primitive>) -> Decorator {
        Decorator { material, child }
    }
}

impl Primitive for Decorator {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let mut hit = self.child.find_first_positive_hit(ray)?;
        hit.material = hit.material.or_else(|| Some(self.material.clone()));
        Some(hit)
    }
}
