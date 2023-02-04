use std::rc::Rc;

use super::primitive::{Hit, LocalPosition, Primitive};
use crate::{math::{pt, vc, CoordinateSystem3D, Point, Ray}, materials::UniformMaterial, imaging::color::Color};

pub struct PlaneXY {}

impl PlaneXY {
    pub fn new() -> Self {
        PlaneXY {}
    }
}

fn compute_uv_coordinates(p: Point<3>) -> Point<2> {
    pt!(p.x(), p.y())
}

fn compute_coordinate_system(origin: Point<3>, ray_origin: &Point<3>) -> CoordinateSystem3D {
    let x_axis = vc!(1, 0, 0);
    let y_axis = vc!(0, 1, 0);
    let z_axis = if ray_origin.z() > 0.0 { vc!(0, 0, 1) } else { vc!(0, 0, -1) };

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

                if t > 0.0 {
                let p = ray.at(t);
                let local_position = LocalPosition {
                    xyz: p,
                    uv: compute_uv_coordinates(p),
                };
                let coordinate_system = compute_coordinate_system(p, &ray.origin);
                let hit = Hit {
                    material_properties: None,
                    t,
                    local_position,
                    transformation: coordinate_system.as_transformation(),
                };

                Some(hit)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::math::approx;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn plane_hit_1() {
        let plane = PlaneXY::new();
        let ray = {
            let origin = pt!(0, 0, 5);
            let direction = vc!(0, 0, -1);
            Ray::new(origin, direction)
        };
        let hit = plane.find_first_positive_hit(&ray).unwrap();

        assert_eq!(approx(pt!(0, 0, 0)), hit.global_position());
        assert_eq!(approx(vc!(0, 0, 1)), hit.normal());
        assert_eq!(approx(vc!(1, 0, 0)), &hit.transformation.matrix * &vc!(1, 0, 0));
        assert_eq!(approx(vc!(0, 1, 0)), &hit.transformation.matrix * &vc!(0, 1, 0));
        assert_eq!(approx(vc!(0, 0, 1)), &hit.transformation.matrix * &vc!(0, 0, 1));
    }

    #[rstest]
    fn plane_hit_2() {
        let plane = PlaneXY::new();
        let ray = {
            let origin = pt!(0, 0, -5);
            let direction = vc!(0, 0, 1);
            Ray::new(origin, direction)
        };
        let hit = plane.find_first_positive_hit(&ray).unwrap();

        assert_eq!(approx(pt!(0, 0, 0)), hit.global_position());
        assert_eq!(approx(vc!(0, 0, -1)), hit.normal());
        assert_eq!(approx(vc!(1, 0, 0)), &hit.transformation.matrix * &vc!(1, 0, 0));
        assert_eq!(approx(vc!(0, 1, 0)), &hit.transformation.matrix * &vc!(0, 1, 0));
        assert_eq!(approx(vc!(0, 0, -1)), &hit.transformation.matrix * &vc!(0, 0, 1));
    }
}