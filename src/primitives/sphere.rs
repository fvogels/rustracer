use super::primitive::{Primitive, Hit, HitPosition};
use crate::math::quadratic::QuadraticEquation;
use crate::math::ray::Ray;
use crate::math::point3d::{Point3D, p3};
use crate::math::vector3d::Vector3D;

pub struct Sphere { }

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

impl Primitive for Sphere {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let delta: Vector3D = ray.origin - p3!(0, 0, 0);
        let radius: f64 = 1.0;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * delta.dot(ray.direction);
        let c = delta.dot(delta) - radius.powi(2);
        let quad_eq = QuadraticEquation::new(a, b, c);

        match quad_eq.solve() {
            None => None,
            Some([t1, t2]) => {
                if t2 < 0.0 {
                    None
                } else {
                    let t = if t1 > 0.0 { t1 } else { t2 };
                    let p = ray.at(t);
                    let position = HitPosition { global: p, local: p };
                    let normal = p - p3!(0, 0, 0);
                    let hit = Hit {
                        t,
                        position,
                        normal,
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
    use super::{Point3D, Vector3D, Ray, p3, Sphere, Primitive};
    use crate::math::vector3d::v3;
    use rstest::rstest;

    #[rstest]
    #[case(p3!(5, 0, 0), v3!(-1, 0, 0), Some(4.0))]
    #[case(p3!(3, 0, 0), v3!(-1, 0, 0), Some(2.0))]
    #[case(p3!(-10, 0, 0), v3!(-1, 0, 0), None)]
    #[case(p3!(10, 5, 0), v3!(-1, 0, 0), None)]
    #[case(p3!(0, 5, 0), v3!(0, -1, 0), Some(4.0))]
    #[case(p3!(0, -8, 0), v3!(0, 1, 0), Some(7.0))]
    #[case(p3!(0, -9, 0), v3!(0, 1, 0), Some(8.0))]
    #[case(p3!(0, -9, 0), v3!(0, 2, 0), Some(4.0))]
    fn first_positive_hit(#[case] ray_origin: Point3D, #[case] ray_direction: Vector3D, #[case] expected_t: Option<f64>) {
        let ray = Ray::new(ray_origin, ray_direction);
        let sphere = Sphere::new();
        let actual_hit = sphere.find_first_positive_hit(&ray);

        assert_eq!(expected_t, actual_hit.map(|hit| hit.t));
    }
}