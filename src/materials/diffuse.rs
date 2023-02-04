use crate::{math::{Ray, Vector, Point, coords::Spherical, Rectangle, Rasterizer, Angle, Interval, IntervalMapper}, util::{Refine}, imaging::color::Color, vc, pt, samplers::{Sampler2D, StratifiedSampler2D}};

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

        Box::new(Result {
            trace,
            accumulated_color: Color::black(),
            sample_count: 0,
            directions: HemisphericalRefiner::new(),
            color_filter: self.color,
            factor: direction.normalized().dot(&Vector::<3>::z_axis()),
        })
    }
}

struct Result {
    trace: TraceFunction,
    directions: HemisphericalRefiner,
    accumulated_color: Color,
    sample_count: u32,
    color_filter: Color,
    factor: f64,
}

impl Refine<Color> for Result {
    fn current(&self) -> Color {
        if self.sample_count == 0 {
            Color::black()
        } else {
            self.accumulated_color * self.color_filter / self.sample_count as f64 * self.factor
        }
    }

    fn refine(&mut self) {
        let direction = self.directions.current();
        let color = self.trace.as_mut()(&direction, 0.1);

        debug_assert!(direction.is_unit());

        let percentage = direction.dot(&Vector::<3>::z_axis()).max(0.0);
        self.accumulated_color += color * percentage;
        self.directions.refine();
        self.sample_count += 1;
    }
}

struct HemisphericalRefiner {
    samples: Box<dyn Refine<Point<2>>>,
    azimuth_mapper: IntervalMapper<f64, Angle>,
    elevation_mapper: IntervalMapper<f64, Angle>,
}

impl HemisphericalRefiner {
    fn new() -> Self {
        HemisphericalRefiner {
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

impl Refine<Vector<3>> for HemisphericalRefiner {
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
        let mut vector = HemisphericalRefiner::new();
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