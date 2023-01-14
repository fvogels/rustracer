use std::rc::Rc;

use super::primitive::{Hit, Primitive};
use crate::lights::light::LightRay;
use crate::math::transformation3d::Transformation3D;
use crate::math::Ray;

pub struct Transformer {
    transformation: Transformation3D,
    child: Rc<dyn Primitive>,
}

impl Transformer {
    pub fn new(transformation: Transformation3D, child: Rc<dyn Primitive>) -> Self {
        Transformer {
            transformation,
            child,
        }
    }
}

impl Primitive for Transformer {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let matrix = &self.transformation.matrix;
        let inverse_matrix = &self.transformation.inverse_matrix;
        let transformed_ray = inverse_matrix * ray;
        let mut hit = self.child.find_first_positive_hit(&transformed_ray)?;
        hit.transformation = self.transformation.compose(&hit.transformation);

        Some(hit)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{
        math::{approx, pt, vc},
        primitives::sphere::Sphere,
    };

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn translated_sphere(
        #[values(-1.2,0.75,0.0,0.2,1.5)] ox: f64,
        #[values(-1.2,0.75,0.0,0.2,1.5)] oy: f64,
        #[values(-1.0,-0.25,0.0,0.25,1.0)] dx: f64,
        #[values(-1.0,-0.25,0.0,0.25,1.0)] dy: f64,
    ) {
        let original = Rc::new(Sphere::new());
        let translation_vector = vc!(1, 0, 0);
        let transformation = Transformation3D::translate(&translation_vector);
        let transformed = Transformer::new(transformation, original.clone());

        let origin1 = pt!(ox, oy, 5);
        let origin2 = &origin1 + &translation_vector;
        let direction = vc!(dx, dy, 1);
        let ray1 = Ray::new(origin1, direction);
        let ray2 = Ray::new(origin2, direction);

        let hit1 = original.find_first_positive_hit(&ray1);
        let hit2 = transformed.find_first_positive_hit(&ray2);

        match (hit1, hit2) {
            (None, None) => {}
            (Some(hit1), Some(hit2)) => {
                assert_eq!(approx(hit1.t), hit2.t);
                assert_eq!(
                    approx(hit1.global_position()),
                    &hit1.global_position() + &translation_vector
                );
            }
            _ => {
                panic!("Different results!")
            }
        }
    }
}
