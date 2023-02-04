use std::rc::Rc;

use super::primitive::{Hit, LocalPosition, Primitive};
use crate::{
    imaging::color::Color,
    math::{
        approx, coords::Cartesian3D, pt, Angle, CoordinateSystem3D, Interval, IntervalMapper,
        Point, QuadraticEquation, Ray,
    }, materials::UniformMaterial,
};

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }
}

fn compute_uv_coordinates(p: &Point<3>) -> Point<2> {
    let cartesian = Cartesian3D {
        x: p.x(),
        y: p.y(),
        z: p.z(),
    };
    let spherical = cartesian.to_spherical();

    debug_assert_eq!(approx(1.0), spherical.radius);

    let azimuth_interval = Interval::new(Angle::degrees(-180.0), Angle::degrees(180.0));
    let elevation_interval = Interval::new(Angle::degrees(-90.0), Angle::degrees(90.0));

    debug_assert!(azimuth_interval.contains(spherical.azimuth));
    debug_assert!(elevation_interval.contains(spherical.elevation));

    let azimuth_mapper = IntervalMapper::new(azimuth_interval, Interval::new(0.0, 1.0));
    let elevation_mapper = IntervalMapper::new(elevation_interval, Interval::new(0.0, 1.0));

    let u = azimuth_mapper.map(spherical.azimuth);
    let v = elevation_mapper.map(spherical.elevation);

    pt!(u, v)
}

fn compute_coordinate_system(origin: Point<3>) -> CoordinateSystem3D {
    let z_axis = origin - pt![0, 0, 0];
    let x_axis = z_axis.orthogonal().normalized();
    let y_axis = x_axis.cross(&z_axis);

    CoordinateSystem3D {
        origin,
        x_axis,
        y_axis,
        z_axis,
    }
}

impl Primitive for Sphere {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let delta = ray.origin - pt!(0, 0, 0);
        let radius: f64 = 1.0;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * delta.dot(&ray.direction);
        let c = delta.dot(&delta) - radius.powi(2);
        let quad_eq = QuadraticEquation::new(a, b, c);

        match quad_eq.solve() {
            None => None,
            Some([t1, t2]) => {
                if t2 < 0.0 {
                    None
                } else {
                    let t = if t1 > 0.0 { t1 } else { t2 };
                    let p = ray.at(t);
                    let local_position = LocalPosition {
                        xyz: p,
                        uv: compute_uv_coordinates(&p),
                    };
                    let coordinate_system = compute_coordinate_system(p);
                    let hit = Hit {
                        t,
                        local_position,
                        transformation: coordinate_system.as_transformation(),
                        material_properties: None,
                    };

                    Some(hit)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::{Point, Primitive, Ray, Sphere};

    #[cfg(test)]
    use crate::math::{pt, vc, Vector};

    use rstest::rstest;

    #[rstest]
    #[case(pt!(5, 0, 0), vc!(-1, 0, 0), Some((4.0, vc!(1, 0, 0))))]
    #[case(pt!(3, 0, 0), vc!(-1, 0, 0), Some((2.0, vc!(1, 0, 0))))]
    #[case(pt!(-10, 0, 0), vc!(-1, 0, 0), None)]
    #[case(pt!(10, 5, 0), vc!(-1, 0, 0), None)]
    #[case(pt!(0, 5, 0), vc!(0, -1, 0), Some((4.0, vc!(0, 1, 0))))]
    #[case(pt!(0, -8, 0), vc!(0, 1, 0), Some((7.0, vc!(0, -1, 0))))]
    #[case(pt!(0, -9, 0), vc!(0, 1, 0), Some((8.0, vc!(0, -1, 0))))]
    #[case(pt!(0, -9, 0), vc!(0, 2, 0), Some((4.0, vc!(0, -1, 0))))]
    fn first_positive_hit(
        #[case] ray_origin: Point<3>,
        #[case] ray_direction: Vector<3>,
        #[case] expected_hit: Option<(f64, Vector<3>)>,
    ) {
        let ray = Ray::new(ray_origin, ray_direction);
        let sphere = Sphere::new();
        let actual_hit = sphere.find_first_positive_hit(&ray);

        match (expected_hit, actual_hit) {
            (None, None) => (),
            (Some(ref expected), Some(ref actual)) => {
                assert_eq!(expected.0, actual.t);
                assert_eq!(expected.1, actual.normal());
            }
            _ => panic!(),
        }
    }
}
