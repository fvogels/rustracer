use super::point3d::Point3D;
use super::vector3d::Vector3D;

pub struct Rectangle3D {
    pub origin: Point3D,
    pub x_axis: Vector3D,
    pub y_axis: Vector3D,
}

impl Rectangle3D {
    pub fn new(origin: Point3D, x_axis: Vector3D, y_axis: Vector3D) -> Rectangle3D {
        Rectangle3D { origin, x_axis, y_axis }
    }
}
