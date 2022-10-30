use super::primitive::{Hit, HitPosition, LocalPosition, Primitive};
use crate::math::{
    point2d::{p2, Point2D},
    point3d::{p3, Point3D},
    ray::Ray,
    vector3d::v3,
};

pub struct PlaneXY {}

impl PlaneXY {
    pub fn new() -> PlaneXY {
        PlaneXY {}
    }
}

fn compute_uv_coordinates(p: &Point3D) -> Point2D {
    p2!(p.x(), p.y())
}

impl Primitive for PlaneXY {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let d = ray.direction.z();

        if d == 0.0 {
            None
        } else {
            let t = (p3!(0, 0, 0) - ray.origin).z() / d;
            let p = ray.at(t);
            let position = HitPosition {
                global: p,
                local: LocalPosition {
                    xyz: p,
                    uv: compute_uv_coordinates(&p),
                },
            };
            let normal = v3!(0, 0, 1);
            let material_properties = None;
            let hit = Hit {
                material_properties,
                normal,
                position,
                t,
            };

            Some(hit)
        }
    }
}
