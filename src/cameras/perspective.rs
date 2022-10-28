use crate::math::{rectangle3d::Rectangle3D, point3d::{Point3D, p3}, vector3d::{Vector3D, v3}};


pub struct PerspectiveCamera {
    screen: Rectangle3D,
}

pub struct PerspectiveCameraParameters {
    eye: Point3D,
    look_at: Point3D,
    up: Vector3D,
    distance_to_screen: f64,
    aspect_ratio: f64,
}

impl PerspectiveCamera {
    pub fn new(parameters: &PerspectiveCameraParameters) -> PerspectiveCamera {
        let screen_width = parameters.aspect_ratio;
        let screen_height = 1.0;
        let origin = p3![-screen_width / 2.0, screen_height / 2.0, parameters.distance_to_screen];
        let x_axis = v3![screen_width, 0, 0];
        let y_axis = v3![0, screen_height, 0];
        let screen = Rectangle3D::new(origin, x_axis, y_axis);

        PerspectiveCamera { screen }
    }
}
