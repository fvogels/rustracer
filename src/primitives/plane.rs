use super::primitive::{Hit, LocalPosition, Primitive};
use crate::math::{pt, vc, CoordinateSystem3D, Point, Ray};

pub struct PlaneXY {}

impl PlaneXY {
    pub fn new() -> Self {
        PlaneXY {}
    }
}

fn compute_uv_coordinates(p: Point<3>) -> Point<2> {
    pt!(p.x(), p.y())
}

fn compute_coordinate_system(origin: Point<3>) -> CoordinateSystem3D {
    let x_axis = vc!(1, 0, 0);
    let y_axis = vc!(0, 1, 0);
    let z_axis = vc!(0, 0, 1);

    CoordinateSystem3D {
        origin,
        x_axis,
        y_axis,
        z_axis,
    }
}

impl Primitive for PlaneXY {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit> {
        let d = ray.direction.z();

        if d == 0.0 {
            None
        } else {
            let t = (pt!(0, 0, 0) - ray.origin).z() / d;
            let p = ray.at(t);
            let local_position = LocalPosition {
                xyz: p,
                uv: compute_uv_coordinates(p),
            };
            let material_properties = None;
            let coordinate_system = compute_coordinate_system(p);
            let hit = Hit {
                material_properties,
                t,
                local_position,
                coordinate_system,
            };

            Some(hit)
        }
    }
}
