use std::rc::Rc;

use crate::math::{transformation3d::Transformation3D, Vector};

use super::{Material, MaterialResult, TraceFunction};

pub struct MaterialTransformer {
    child: Rc<dyn Material>,
    transformation: Transformation3D,
}

impl MaterialTransformer {
    pub fn new(child: Rc<dyn Material>, transformation: Transformation3D) -> Self {
        MaterialTransformer {
            child,
            transformation,
        }
    }
}

impl Material for MaterialTransformer {
    fn at(&self, direction: &Vector<3>, trace: TraceFunction) -> MaterialResult {
        let transformed_direction = &self.transformation.inverse_matrix * direction;
        let transformed_trace: TraceFunction = Box::new({
            let copy = self.transformation.matrix.clone();

            move |d, w| {
                let transformed_d = &copy * d;
                trace(&transformed_d, w)
            }
        });

        self.child.at(&transformed_direction, Box::new(transformed_trace))
    }
}
