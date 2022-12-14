use crate::{
    materials::material::MaterialProperties,
    math::{Point, Ray, CoordinateSystem3D, Vector},
};

pub struct Hit {
    pub t: f64,
    pub local_position: LocalPosition,
    pub coordinate_system: CoordinateSystem3D,
    pub material_properties: Option<MaterialProperties>,
}

#[derive(Copy, Clone)]
pub struct LocalPosition {
    pub xyz: Point<3>,
    pub uv: Point<2>,
}

pub trait Primitive {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit>;
}

impl Hit {
    pub fn smallest_positive(hit1: Hit, hit2: Hit) -> Option<Hit> {
        if hit1.t < 0.0 {
            if hit2.t < 0.0 {
                debug_assert!(hit1.t < 0.0 && hit2.t < 0.0);
                None
            } else {
                debug_assert!(hit1.t < 0.0 && hit2.t >= 0.0);
                Some(hit2)
            }
        } else if hit2.t < 0.0 {
            debug_assert!(hit1.t >= 0.0 && hit2.t < 0.0);
            Some(hit1)
        } else {
            debug_assert!(hit1.t >= 0.0 && hit2.t >= 0.0);
            if hit1.t < hit2.t {
                Some(hit1)
            } else {
                Some(hit2)
            }
        }
    }

    pub fn normal(&self) -> Vector<3> {
        self.coordinate_system.z_axis
    }

    pub fn global_position(&self) -> Point<3> {
        self.coordinate_system.origin
    }
}
