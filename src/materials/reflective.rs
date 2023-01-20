use crate::{math::{Ray, Vector, Point}, util::Constant};

use super::material::{Material, MaterialResult, TraceFunction};

pub struct ReflectiveMaterial {
    reflectivity: f64
}

impl ReflectiveMaterial {
    pub fn new(reflectivity: f64) -> Self {
        ReflectiveMaterial { reflectivity }
    }
}

impl Material for ReflectiveMaterial {
    fn at(&self, direction: &Vector<3>, trace: TraceFunction) -> MaterialResult {
        debug_assert!(direction.z() > 0.0, "Direction {:?} should point outwards", direction);

        let reflected_direction = (-direction).reflect(&Vector::<3>::z_axis());
        let color = trace(&reflected_direction, self.reflectivity);

        Box::new(Constant::new(color * self.reflectivity))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{pt, vc, imaging::color::Color, math::approx};

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn test() {
        let material = ReflectiveMaterial::new(1.0);
        let direction = vc!(-1, 0, 1);
        let trace: TraceFunction = {
            let expected_trace_direction= vc!(1, 0, 1);

            Box::new(move |direction: &Vector<3>, w: f64| {
                assert_eq!(approx(expected_trace_direction), *direction);
                Color::black()
            })
        };

        material.at(&direction, trace);
    }
}