use std::rc::Rc;

use crate::{materials::{Material}, math::Ray};

use super::primitive::{Hit, Primitive};

pub struct Decorator {
    material: Rc<dyn Material>,
    child: Rc<dyn Primitive>,
}

impl Decorator {
    pub fn new(material: Rc<dyn Material>, child: Rc<dyn Primitive>) -> Self {
        Decorator { material, child }
    }
}

impl Primitive for Decorator {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let mut hit = self.child.find_first_positive_hit(ray)?;
        hit.material_properties = Some(self.material.at(hit.local_position));
        Some(hit)
    }
}
