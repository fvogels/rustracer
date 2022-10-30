use crate::math::{
    matrix4d::Matrix4D,
    point2d::Point2D,
    point3d::{p3, Point3D},
    ray::Ray,
    rectangle3d::Rectangle3D,
    vector3d::{v3, Vector3D},
};

pub struct PerspectiveCamera {
    screen: Rectangle3D,
    transformation_matrix: Matrix4D,
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

fn create_coordinate_system(parameters: &PerspectiveCameraParameters) -> (Point3D, Vector3D, Vector3D, Vector3D) {
    let look_direction = (parameters.look_at - parameters.eye).normalized();
    let right = look_direction.cross(&parameters.up).normalized();
    let up = right.cross(&look_direction);

    debug_assert!(look_direction.is_unit());
    debug_assert!(right.is_unit());
    debug_assert!(up.is_unit());
    debug_assert!(look_direction.is_orthogonal_to(&right));

    let origin = parameters.eye;
    let z_axis = -look_direction;
    let y_axis = up;
    let x_axis = right;

    debug_assert!(x_axis.is_unit());
    debug_assert!(y_axis.is_unit());
    debug_assert!(z_axis.is_unit());
    debug_assert!(x_axis.is_orthogonal_to(&y_axis));
    debug_assert!(x_axis.is_orthogonal_to(&z_axis));
    debug_assert!(y_axis.is_orthogonal_to(&z_axis));

    (origin, x_axis, y_axis, z_axis)
}

fn create_transformation_matrix(parameters: &PerspectiveCameraParameters) -> Matrix4D {
    let (origin, x_axis, y_axis, z_axis) = create_coordinate_system(parameters);

    Matrix4D::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
}

fn create_canonical_screen(parameters: &PerspectiveCameraParameters) -> Rectangle3D {
    let screen_width = parameters.aspect_ratio;
    let screen_height = 1.0;
    let origin = p3![
        -screen_width / 2.0,
        screen_height / 2.0,
        -parameters.distance_to_screen
    ];
    let x_axis = v3![screen_width, 0, 0];
    let y_axis = v3![0, -screen_height, 0];

    Rectangle3D::new(origin, x_axis, y_axis)
}

impl PerspectiveCamera {
    pub fn new(parameters: &PerspectiveCameraParameters) -> PerspectiveCamera {
        let screen = create_canonical_screen(parameters);
        let transformation_matrix = create_transformation_matrix(parameters);

        PerspectiveCamera {
            screen,
            transformation_matrix,
        }
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
            let transformed_ray = &self.parent.transformation_matrix * &ray;

            Some(transformed_ray)
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use rstest::rstest;

    #[cfg(test)]
    use crate::math::{approx::approx, point2d::p2};

    #[rstest]
    fn coordinate_system_1() {
        let eye = p3!(0, 0, 0);
        let look_at = p3!(0, 0, 1);
        let up = v3!(0, 1, 0);
        let expected_origin = p3!(0, 0, 0);
        let expected_x_axis = v3!(-1, 0, 0);
        let expected_y_axis = v3!(0, 1, 0);
        let expected_z_axis = v3!(0, 0, -1);

        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye,
            look_at,
            up,
        };

        let (actual_origin, actual_x_axis, actual_y_axis, actual_z_axis) = create_coordinate_system(&parameters);

        assert_eq!(approx(expected_origin), actual_origin);
        assert_eq!(approx(expected_x_axis), actual_x_axis);
        assert_eq!(approx(expected_y_axis), actual_y_axis);
        assert_eq!(approx(expected_z_axis), actual_z_axis);
    }

    #[rstest]
    fn coordinate_system_2() {
        let eye = p3!(0, 0, 1);
        let look_at = p3!(0, 0, 0);
        let up = v3!(0, 1, 0);
        let expected_origin = p3!(0, 0, 1);
        let expected_x_axis = v3!(1, 0, 0);
        let expected_y_axis = v3!(0, 1, 0);
        let expected_z_axis = v3!(0, 0, 1);

        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye,
            look_at,
            up,
        };

        let (actual_origin, actual_x_axis, actual_y_axis, actual_z_axis) = create_coordinate_system(&parameters);

        assert_eq!(approx(expected_origin), actual_origin);
        assert_eq!(approx(expected_x_axis), actual_x_axis);
        assert_eq!(approx(expected_y_axis), actual_y_axis);
        assert_eq!(approx(expected_z_axis), actual_z_axis);
    }

    #[rstest]
    fn coordinate_system_3() {
        let eye = p3!(0, 0, -1);
        let look_at = p3!(0, 0, 0);
        let up = v3!(0, 1, 0);
        let expected_origin = p3!(0, 0, -1);
        let expected_x_axis = v3!(-1, 0, 0);
        let expected_y_axis = v3!(0, 1, 0);
        let expected_z_axis = v3!(0, 0, -1);

        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye,
            look_at,
            up,
        };

        let (actual_origin, actual_x_axis, actual_y_axis, actual_z_axis) = create_coordinate_system(&parameters);

        assert_eq!(approx(expected_origin), actual_origin);
        assert_eq!(approx(expected_x_axis), actual_x_axis);
        assert_eq!(approx(expected_y_axis), actual_y_axis);
        assert_eq!(approx(expected_z_axis), actual_z_axis);
    }

    #[rstest]
    #[case(p2!(0.5, 0.5), p3!(0, 0, 0))]
    #[case(p2!(0, 0), p3!(0.5, 0.5, 0))]
    #[case(p2!(1, 0), p3!(-0.5, 0.5, 0))]
    #[case(p2!(0, 1), p3!(0.5, -0.5, 0))]
    #[case(p2!(1, 1), p3!(-0.5, -0.5, 0))]
    fn enumerate_rays_1(#[case] p: Point2D, #[case] expected: Point3D) {
        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye: p3!(0, 0, -1),
            look_at: p3!(0, 0, 0),
            up: v3!(0, 1, 0),
        };
        let camera = PerspectiveCamera::new(&parameters);
        let rays: Vec<Ray> = camera.enumerate_rays(p).collect();

        assert_eq!(1, rays.len());

        let ray = &rays[0];
        assert_eq!(approx(p3!(0, 0, -1)), ray.origin);
        assert_eq!(approx(expected), ray.at(1.0));
    }

    #[rstest]
    #[case(p2!(0.5, 0.5), p3!(0, 0, 1))]
    #[case(p2!(0, 0), p3!(-0.5, 0.5, 1))]
    #[case(p2!(1, 0), p3!(0.5, 0.5, 1))]
    #[case(p2!(0, 1), p3!(-0.5, -0.5, 1))]
    #[case(p2!(1, 1), p3!(0.5, -0.5, 1))]
    fn enumerate_rays_2(#[case] p: Point2D, #[case] expected: Point3D) {
        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye: p3!(0, 0, 2),
            look_at: p3!(0, 0, 0),
            up: v3!(0, 1, 0),
        };
        let camera = PerspectiveCamera::new(&parameters);
        let rays: Vec<Ray> = camera.enumerate_rays(p).collect();

        assert_eq!(1, rays.len());

        let ray = &rays[0];
        assert_eq!(approx(p3!(0, 0, 2)), ray.origin);
        assert_eq!(approx(expected), ray.at(1.0));
    }
}
