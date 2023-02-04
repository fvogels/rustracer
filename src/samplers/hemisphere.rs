use crate::{util::Refine, samplers::{Sampler2D, StratifiedSampler2D}, vc, pt};

use crate::math::{Point, IntervalMapper, Angle, Vector, Interval, Rectangle, coords::Spherical};

pub struct HemisphereSampler {
    samples: Box<dyn Refine<Point<2>>>,
    azimuth_mapper: IntervalMapper<f64, Angle>,
    elevation_mapper: IntervalMapper<f64, Angle>,
}

impl HemisphereSampler {
    pub fn new() -> Self {
        HemisphereSampler {
            samples: Self::create_samples(),
            azimuth_mapper: Self::create_azimuth_mapper(),
            elevation_mapper: Self::create_elevation_mapper(),
        }
    }

    fn create_azimuth_mapper() -> IntervalMapper<f64, Angle> {
        let start = Angle::degrees(0.0);
        let end = Angle::degrees(180.0);
        let azimuth_interval = Interval::new(start, end);
        let unit_interval = Interval::new(0.0, 1.0);
        IntervalMapper::new(unit_interval, azimuth_interval)
    }

    fn create_elevation_mapper() -> IntervalMapper<f64, Angle> {
        let start = Angle::degrees(-90.0);
        let end = Angle::degrees(90.0);
        let azimuth_interval = Interval::new(start, end);
        let unit_interval = Interval::new(0.0, 1.0);
        IntervalMapper::new(unit_interval, azimuth_interval)
    }

    fn create_samples() -> Box<dyn Refine<Point<2>>> {
        let rectangle = Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1));
        let sampler = StratifiedSampler2D::new();
        sampler.sample(rectangle)
    }
}

impl Refine<Vector<3>> for HemisphereSampler {
    fn current(&self) -> Vector<3> {
        let sample = self.samples.current();
        let x = sample.x();
        let y = sample.y();
        let spherical_coordinates = Spherical {
            radius: 1.0,
            azimuth: self.azimuth_mapper.map(x),
            elevation: self.elevation_mapper.map(y),
        };
        let cartesian_coordinates = spherical_coordinates.to_cartesian3d();

        let x = cartesian_coordinates.x;
        let y = cartesian_coordinates.y;
        let z = cartesian_coordinates.z;

        vc!(x, y, z)
    }

    fn refine(&mut self) {
        self.samples.refine()
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::math::approx;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn hemispherical_refiner() {
        let mut vector = HemisphereSampler::new();
        let constant = 2.0f64.sqrt() / 2.0;

        assert_eq!(approx(vc!(0, 0, 1)), vector.current());
        vector.refine();
        assert_eq!(approx(vc!(0.5, -constant, 0.5)), vector.current());
        vector.refine();
        assert_eq!(approx(vc!(-0.5, -constant, 0.5)), vector.current());
        vector.refine();
        assert_eq!(approx(vc!(0.5, constant, 0.5)), vector.current());
        vector.refine();
        assert_eq!(approx(vc!(-0.5, constant, 0.5)), vector.current());
    }
}