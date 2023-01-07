use crate::{
    imaging::color::Color,
    math::{Point, Ray},
};

use super::light::{LightRay, LightSource};

pub struct PointLight {
    color: Color,
    position: Point<3>,
}

pub struct PointLightIterator {
    lightray: Option<LightRay>,
}

impl PointLight {
    pub fn new(color: Color, position: Point<3>) -> Self {
        PointLight { color, position }
    }
}

impl LightSource for PointLight {
    fn lightrays_to(&self, point: Point<3>) -> Box<dyn Iterator<Item = LightRay>> {
        let ray = Ray::through(self.position, point);
        let lightray = LightRay::new(self.color, ray);
        let iterator = PointLightIterator {
            lightray: Some(lightray),
        };

        Box::new(iterator)
    }
}

impl Iterator for PointLightIterator {
    type Item = LightRay;

    fn next(&mut self) -> Option<Self::Item> {
        self.lightray.take()
    }
}
