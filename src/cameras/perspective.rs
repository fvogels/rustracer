use crate::math::{pt, vc, Matrix, Point, Ray, Rectangle, Vector};

pub struct PerspectiveCamera {
    screen: Rectangle<3>,
    transformation_matrix: Matrix<4, 4>,
}

pub struct PerspectiveCameraParameters {
    pub eye: Point<3>,
    pub look_at: Point<3>,
    pub up: Vector<3>,
    pub distance_to_screen: f64,
    pub aspect_ratio: f64,
}

pub struct Rays<'a> {
    parent: &'a PerspectiveCamera,
    point: Point<2>,
    consumed: bool,
}

fn create_coordinate_system(
    parameters: &PerspectiveCameraParameters,
) -> (Point<3>, Vector<3>, Vector<3>, Vector<3>) {
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

fn create_transformation_matrix(parameters: &PerspectiveCameraParameters) -> Matrix<4, 4> {
    let (origin, x_axis, y_axis, z_axis) = create_coordinate_system(parameters);

    Matrix::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
}

fn create_canonical_screen(parameters: &PerspectiveCameraParameters) -> Rectangle<3> {
    let screen_width = parameters.aspect_ratio;
    let screen_height = 1.0;
    let origin = pt![
        -screen_width / 2.0,
        screen_height / 2.0,
        -parameters.distance_to_screen
    ];
    let x_axis = vc![screen_width, 0, 0];
    let y_axis = vc![0, -screen_height, 0];

    Rectangle::<3>::new(origin, x_axis, y_axis)
}

impl PerspectiveCamera {
    pub fn new(parameters: &PerspectiveCameraParameters) -> Self {
        let screen = create_canonical_screen(parameters);
        let transformation_matrix = create_transformation_matrix(parameters);

        PerspectiveCamera {
            screen,
            transformation_matrix,
        }
    }

    pub fn enumerate_rays(&self, point: Point<2>) -> Rays {
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

            let origin = pt!(0, 0, 0);
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
    use crate::math::{approx, pt, vc};

    #[rstest]
    fn coordinate_system_1() {
        let eye = pt!(0, 0, 0);
        let look_at = pt!(0, 0, 1);
        let up = vc!(0, 1, 0);
        let expected_origin = pt!(0, 0, 0);
        let expected_x_axis = vc!(-1, 0, 0);
        let expected_y_axis = vc!(0, 1, 0);
        let expected_z_axis = vc!(0, 0, -1);

        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye,
            look_at,
            up,
        };

        let (actual_origin, actual_x_axis, actual_y_axis, actual_z_axis) =
            create_coordinate_system(&parameters);

        assert_eq!(approx(expected_origin), actual_origin);
        assert_eq!(approx(expected_x_axis), actual_x_axis);
        assert_eq!(approx(expected_y_axis), actual_y_axis);
        assert_eq!(approx(expected_z_axis), actual_z_axis);
    }

    #[rstest]
    fn coordinate_system_2() {
        let eye = pt!(0, 0, 1);
        let look_at = pt!(0, 0, 0);
        let up = vc!(0, 1, 0);
        let expected_origin = pt!(0, 0, 1);
        let expected_x_axis = vc!(1, 0, 0);
        let expected_y_axis = vc!(0, 1, 0);
        let expected_z_axis = vc!(0, 0, 1);

        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye,
            look_at,
            up,
        };

        let (actual_origin, actual_x_axis, actual_y_axis, actual_z_axis) =
            create_coordinate_system(&parameters);

        assert_eq!(approx(expected_origin), actual_origin);
        assert_eq!(approx(expected_x_axis), actual_x_axis);
        assert_eq!(approx(expected_y_axis), actual_y_axis);
        assert_eq!(approx(expected_z_axis), actual_z_axis);
    }

    #[rstest]
    fn coordinate_system_3() {
        let eye = pt!(0, 0, -1);
        let look_at = pt!(0, 0, 0);
        let up = vc!(0, 1, 0);
        let expected_origin = pt!(0, 0, -1);
        let expected_x_axis = vc!(-1, 0, 0);
        let expected_y_axis = vc!(0, 1, 0);
        let expected_z_axis = vc!(0, 0, -1);

        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye,
            look_at,
            up,
        };

        let (actual_origin, actual_x_axis, actual_y_axis, actual_z_axis) =
            create_coordinate_system(&parameters);

        assert_eq!(approx(expected_origin), actual_origin);
        assert_eq!(approx(expected_x_axis), actual_x_axis);
        assert_eq!(approx(expected_y_axis), actual_y_axis);
        assert_eq!(approx(expected_z_axis), actual_z_axis);
    }

    #[rstest]
    #[case(pt!(0.5, 0.5), pt!(0, 0, 0))]
    #[case(pt!(0, 0), pt!(0.5, 0.5, 0))]
    #[case(pt!(1, 0), pt!(-0.5, 0.5, 0))]
    #[case(pt!(0, 1), pt!(0.5, -0.5, 0))]
    #[case(pt!(1, 1), pt!(-0.5, -0.5, 0))]
    fn enumerate_rays_1(#[case] p: Point<2>, #[case] expected: Point<3>) {
        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye: pt!(0, 0, -1),
            look_at: pt!(0, 0, 0),
            up: vc!(0, 1, 0),
        };
        let camera = PerspectiveCamera::new(&parameters);
        let rays: Vec<Ray> = camera.enumerate_rays(p).collect();

        assert_eq!(1, rays.len());

        let ray = &rays[0];
        assert_eq!(approx(pt!(0, 0, -1)), ray.origin);
        assert_eq!(approx(expected), ray.at(1.0));
    }

    #[rstest]
    #[case(pt!(0.5, 0.5), pt!(0, 0, 1))]
    #[case(pt!(0, 0), pt!(-0.5, 0.5, 1))]
    #[case(pt!(1, 0), pt!(0.5, 0.5, 1))]
    #[case(pt!(0, 1), pt!(-0.5, -0.5, 1))]
    #[case(pt!(1, 1), pt!(0.5, -0.5, 1))]
    fn enumerate_rays_2(#[case] p: Point<2>, #[case] expected: Point<3>) {
        let parameters = PerspectiveCameraParameters {
            aspect_ratio: 1.0,
            distance_to_screen: 1.0,
            eye: pt!(0, 0, 2),
            look_at: pt!(0, 0, 0),
            up: vc!(0, 1, 0),
        };
        let camera = PerspectiveCamera::new(&parameters);
        let rays: Vec<Ray> = camera.enumerate_rays(p).collect();

        assert_eq!(1, rays.len());

        let ray = &rays[0];
        assert_eq!(approx(pt!(0, 0, 2)), ray.origin);
        assert_eq!(approx(expected), ray.at(1.0));
    }
}
