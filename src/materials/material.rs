use crate::{imaging::color::Color, math::Ray, util::Refine};

pub type MaterialResult = Box<dyn Refine<Color>>;

pub trait Material {
    fn at(&self, ray: Ray, trace: Box<dyn Fn(Ray) -> Color>) -> MaterialResult;
}
