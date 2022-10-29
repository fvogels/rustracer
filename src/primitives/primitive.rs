use crate::math::{point3d::Point3D, ray::Ray, vector3d::Vector3D, point2d::Point2D};

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f64,
    pub position: HitPosition,
    pub normal: Vector3D,
}

#[derive(Copy, Clone)]
pub struct HitPosition {
    pub global: Point3D,
    pub local: LocalHitPosition,
}

#[derive(Copy, Clone)]
pub struct LocalHitPosition {
    pub xyz: Point3D,
    pub uv: Point2D,
}

pub trait Primitive {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit>;
}

impl Hit {
    pub fn overwrite_if_closer(&mut self, hit: &Hit) {
        debug_assert!(self.t >= 0.0);
        debug_assert!(hit.t >= 0.0);

        if hit.t < self.t {
            *self = *hit;
        }
    }
}
