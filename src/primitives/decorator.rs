use std::rc::Rc;

use crate::{materials::material::Material, math::ray::Ray};

use super::primitive::{Hit, Primitive};

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
        hit.material_properties = hit
            .material_properties
            .or_else(|| Some(self.material.at(hit.position.local)));
        Some(hit)
    }
}
