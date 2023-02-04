use crate::math::Vector;

pub trait BRDF {
    fn compute(&self, outgoing: &Vector<3>, incoming: &Vector<3>) -> f64;
}

pub struct Diffuse { }

impl Diffuse {
    pub fn new() -> Self {
        Diffuse { }
    }
}

impl BRDF for Diffuse {
    fn compute(&self, outgoing: &Vector<3>, incoming: &Vector<3>) -> f64 {
        1.0
    }
}