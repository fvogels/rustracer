use crate::{math::{Ray, Vector, Point, coords::Spherical, Rectangle, Rasterizer, Angle, Interval, IntervalMapper}, util::Constant, imaging::color::Color, vc, pt, samplers::{Sampler2D, StratifiedSampler2D}};

use super::material::{Material, MaterialResult, TraceFunction};

pub struct DiffuseMaterial {
    color: Color,
}

impl DiffuseMaterial {
    pub fn new(color: Color) -> Self {
        DiffuseMaterial { color }
    }
}

impl Material for DiffuseMaterial {
    fn at(&self, direction: &Vector<3>, trace: TraceFunction) -> MaterialResult {
        debug_assert!(direction.z() > 0.0, "Direction {:?} should point outwards", direction);

        let rectangle = Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1));
        let sampler = StratifiedSampler2D::new();
        let samples = sampler.sample(rectangle);
        let azimuth_mapper = {
            let start = Angle::degrees(0.0);
            let end = Angle::degrees(360.0);
            let azimuth_interval = Interval::new(start, end);
            let unit_interval = Interval::new(0.0, 1.0);
            IntervalMapper::new(unit_interval, azimuth_interval)
        };
        let elevation_mapper = {
            let start = Angle::degrees(0.0);
            let end = Angle::degrees(90.0);
            let azimuth_interval = Interval::new(start, end);
            let unit_interval = Interval::new(0.0, 1.0);
            IntervalMapper::new(unit_interval, azimuth_interval)
        };

        let mut total = Color::black();
        let sample_count = 2;
        for sample in samples.take(sample_count) {
            let radius = 1.0;
            let azimuth = azimuth_mapper.map(sample.x());
            let elevation = elevation_mapper.map(sample.y());
            let spherical = Spherical { radius, azimuth, elevation };
            let cartesian = spherical.to_cartesian3d();
            let direction = vc!(cartesian.x, cartesian.y, cartesian.z);

            total += trace(&direction, 0.9);
        }

        let result = total * self.color / sample_count as f64;

        Box::new(Constant::new(result))
    }
}
