use super::primitive::{Hit, HitPosition, Primitive, LocalPosition};
use crate::math::{approx::approx, coords::Cartesian3D, point2d::{p2, Point2D}, point3d::{p3, Point3D}, quadratic::QuadraticEquation, ray::Ray, vector3d::Vector3D, interval::{Interval, IntervalMapper}, angle::Angle};

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

fn compute_uv_coordinates(p: &Point3D) -> Point2D {
    let cartesian = Cartesian3D { x: p.x(), y: p.y(), z: p.z() };
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

    p2!(u, v)
}

impl Primitive for Sphere {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let delta: Vector3D = ray.origin - p3!(0, 0, 0);
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
                    let position = HitPosition {
                        global: p,
                        local: LocalPosition { xyz: p, uv: compute_uv_coordinates(&p) },
                    };
                    let normal = p - p3!(0, 0, 0);
                    let hit = Hit {
                        t,
                        position,
                        normal,
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
    use super::{p3, Point3D, Primitive, Ray, Sphere, Vector3D};
    use crate::math::vector3d::v3;
    use rstest::rstest;

    #[rstest]
    #[case(p3!(5, 0, 0), v3!(-1, 0, 0), Some((4.0, v3!(1, 0, 0))))]
    #[case(p3!(3, 0, 0), v3!(-1, 0, 0), Some((2.0, v3!(1, 0, 0))))]
    #[case(p3!(-10, 0, 0), v3!(-1, 0, 0), None)]
    #[case(p3!(10, 5, 0), v3!(-1, 0, 0), None)]
    #[case(p3!(0, 5, 0), v3!(0, -1, 0), Some((4.0, v3!(0, 1, 0))))]
    #[case(p3!(0, -8, 0), v3!(0, 1, 0), Some((7.0, v3!(0, -1, 0))))]
    #[case(p3!(0, -9, 0), v3!(0, 1, 0), Some((8.0, v3!(0, -1, 0))))]
    #[case(p3!(0, -9, 0), v3!(0, 2, 0), Some((4.0, v3!(0, -1, 0))))]
    fn first_positive_hit(
        #[case] ray_origin: Point3D,
        #[case] ray_direction: Vector3D,
        #[case] expected_hit: Option<(f64, Vector3D)>,
    ) {
        let ray = Ray::new(ray_origin, ray_direction);
        let sphere = Sphere::new();
        let actual_hit = sphere.find_first_positive_hit(&ray);

        match (expected_hit, actual_hit) {
            (None, None) => (),
            (Some(ref expected), Some(ref actual)) => {
                assert_eq!(expected.0, actual.t);
                assert_eq!(expected.1, actual.normal);
            }
            _ => panic!(),
        }
    }
}
