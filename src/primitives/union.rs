use std::rc::Rc;

use super::primitive::{Hit, Primitive};
use crate::math::ray::Ray;
use crate::math::transformation3d::Transformation3D;

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
            match (result, child.find_first_positive_hit(ray)) {
                (_, None) => { },
                (None, Some(h)) => { result = Some(h); },
                (Some(mut h1), Some(h2)) => {
                    h1.overwrite_if_closer(&h2);
                }
            }
        }

        result
    }
}
