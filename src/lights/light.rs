use crate::{
    imaging::color::Color,
    math::{Point, Ray},
};

pub trait LightSource {
    fn lightrays_to(&self, point: Point<3>) -> Box<dyn Iterator<Item = LightRay>>;
}

pub struct LightRay {
    pub color: Color,
    pub ray: Ray,
}

impl LightRay {
    pub fn new(color: Color, ray: Ray) -> Self {
        LightRay { color, ray }
    }
}
