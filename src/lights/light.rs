use crate::{math::{ray::Ray, point3d::Point3D}, imaging::color::Color};

pub trait LightSource {
    fn lightrays_to(&self, point: Point3D) -> Box<dyn Iterator<Item=LightRay>>;
}

pub struct LightRay {
    pub color: Color,
    pub ray: Ray,
}

impl LightRay {
    pub fn new(color: Color, ray: Ray) -> LightRay {
        LightRay { color, ray }
    }
}