use super::point2d::Point2D;
use super::point3d::Point3D;
use super::vector3d::Vector3D;

pub struct Rectangle3D {
    pub origin: Point3D,
    pub x_axis: Vector3D,
    pub y_axis: Vector3D,
}

impl Rectangle3D {
    pub fn new(origin: Point3D, x_axis: Vector3D, y_axis: Vector3D) -> Rectangle3D {
        Rectangle3D {
            origin,
            x_axis,
            y_axis,
        }
    }

    pub fn from_relative(&self, p: Point2D) -> Point3D {
        self.origin + self.x_axis * p.x() + self.y_axis * p.y()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use crate::math::point2d::p2;
    use crate::math::point3d::p3;
    use crate::math::vector3d::v3;
    use rstest::rstest;

    #[rstest]
    #[case(Rectangle3D::new(p3!(0, 0, 0), v3!(1, 0, 0), v3!(0, 1, 0)), p2!(0, 0), p3!(0, 0, 0))]
    #[case(Rectangle3D::new(p3!(0, 0, 0), v3!(1, 0, 0), v3!(0, 1, 0)), p2!(1, 0), p3!(1, 0, 0))]
    #[case(Rectangle3D::new(p3!(0, 0, 0), v3!(1, 0, 0), v3!(0, 1, 0)), p2!(0, 1), p3!(0, 1, 0))]
    #[case(Rectangle3D::new(p3!(0, 0, 0), v3!(1, 0, 0), v3!(0, 1, 0)), p2!(1, 1), p3!(1, 1, 0))]
    #[case(Rectangle3D::new(p3!(0, 0, 0), v3!(2, 0, 0), v3!(0, 4, 0)), p2!(1, 0), p3!(2, 0, 0))]
    #[case(Rectangle3D::new(p3!(0, 0, 0), v3!(2, 0, 0), v3!(0, 4, 0)), p2!(0, 1), p3!(0, 4, 0))]
    fn from_relative(
        #[case] rectangle: Rectangle3D,
        #[case] p: Point2D,
        #[case] expected: Point3D,
    ) {
        let actual = rectangle.from_relative(p);

        assert_eq!(expected, actual);
    }
}
