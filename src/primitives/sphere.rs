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
    fn find_first_positive_hit(ray: &Ray) -> Option<Hit> {
        let delta: Vector3D = ray.origin - p3!(0.0, 0.0, 0.0);
        let radius: f64 = 1.0;
        let a = ray.direction.dot(ray.direction);
        let b = delta.dot(ray.direction);
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
                    let n = p - p3!(0, 0, 0);
                    let hit = Hit {
                        position: HitPosition { global: p, local: p },
                        normal: n,
                    };

                    Some(hit)
                }
            }
        }
    }
}
