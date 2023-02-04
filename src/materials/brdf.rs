use crate::math::Vector;

pub trait BRDF {
    fn compute(&self, outgoing: &Vector<3>, incoming: &Vector<3>) -> f64;
}
