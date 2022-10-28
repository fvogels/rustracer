use crate::math::point3d::Point3D;
use crate::math::ray::Ray;
use crate::math::vector3d::Vector3D;

pub struct Hit {
    pub t: f64,
    pub position: HitPosition,
    pub normal: Vector3D,
}

pub struct HitPosition {
    pub global: Point3D,
    pub local: Point3D,
}

pub trait Primitive {
    fn find_first_positive_hit(&self, ray: &Ray) -> Option<Hit>;
}
