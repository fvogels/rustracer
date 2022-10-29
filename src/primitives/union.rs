use std::rc::Rc;

use super::primitive::{Hit, Primitive};
use crate::math::ray::Ray;

pub struct Union {
    children: Vec<Rc<dyn Primitive>>,
}

impl Union {
    pub fn new(children: Vec<Rc<dyn Primitive>>) -> Union {
        Union { children }
    }
}

impl Primitive for Union {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let mut result: Option<Hit> = None;

        for child in self.children.iter() {
            match (std::mem::take(&mut result), child.find_first_positive_hit(ray)) {
                (None, None) => { },
                (Some(h), None) => { result = Some(h); }
                (None, Some(h)) => { result = Some(h); },
                (Some(h1), Some(h2)) => {
                    result = Hit::smallest_positive(h1, h2)
                }
            }
        }

        result
    }
}
