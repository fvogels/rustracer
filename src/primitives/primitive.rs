use crate::math::vector3d::Vector3D;
use crate::math::point3d::Point3D;
use crate::math::ray::Ray;

pub struct Hit {
    pub position: HitPosition,
    pub normal: Vector3D,
}

pub struct HitPosition {
    pub global: Point3D,
    pub local: Point3D,
}

pub trait Primitive {
    fn find_first_positive_hit(ray: &Ray) -> Hit;
}
