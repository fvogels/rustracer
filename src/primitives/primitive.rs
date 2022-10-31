use crate::{
    materials::material::{MaterialProperties},
    math::{point2d::Point2D, point3d::Point3D, ray::Ray, vector3d::Vector3D},
};

pub struct Hit {
    pub t: f64,
    pub position: HitPosition,
    pub normal: Vector3D,
    pub material_properties: Option<MaterialProperties>,
}

#[derive(Copy, Clone)]
pub struct HitPosition {
    pub global: Point3D,
    pub local: LocalPosition,
}

#[derive(Copy, Clone)]
pub struct LocalPosition {
    pub xyz: Point3D,
    pub uv: Point2D,
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
}
