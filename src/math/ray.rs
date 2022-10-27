use super::point3d::Point3D;
use super::vector3d::Vector3D;


pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }
}
