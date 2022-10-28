use crate::math::{
    point2d::Point2D,
    point3d::{p3, Point3D},
    ray::Ray,
    rectangle3d::Rectangle3D,
    vector3d::{v3, Vector3D},
};

pub struct PerspectiveCamera {
    screen: Rectangle3D,
}

pub struct PerspectiveCameraParameters {
    pub eye: Point3D,
    pub look_at: Point3D,
    pub up: Vector3D,
    pub distance_to_screen: f64,
    pub aspect_ratio: f64,
}

pub struct Rays<'a> {
    parent: &'a PerspectiveCamera,
    point: Point2D,
    consumed: bool,
}

impl PerspectiveCamera {
    pub fn new(parameters: &PerspectiveCameraParameters) -> PerspectiveCamera {
        let screen_width = parameters.aspect_ratio;
        let screen_height = 1.0;
        let origin = p3![
            -screen_width / 2.0,
            screen_height / 2.0,
            parameters.distance_to_screen
        ];
        let x_axis = v3![screen_width, 0, 0];
        let y_axis = v3![0, screen_height, 0];
        let screen = Rectangle3D::new(origin, x_axis, y_axis);

        PerspectiveCamera { screen }
    }

    pub fn enumerate_rays(&self, point: Point2D) -> Rays {
        Rays {
            parent: self,
            point,
            consumed: false,
        }
    }
}

impl<'a> Iterator for Rays<'a> {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            None
        } else {
            self.consumed = true;

            let origin = p3!(0, 0, 0);
            let to = self.parent.screen.from_relative(&self.point);
            let direction = to - origin;
            let ray = Ray::new(origin, direction);

            Some(ray)
        }
    }
}
