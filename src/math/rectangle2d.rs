use super::point2d::{p2, Point2D};
use super::vector2d::Vector2D;

pub struct Rectangle2D {
    pub origin: Point2D,
    pub x_axis: Vector2D,
    pub y_axis: Vector2D,
}

impl Rectangle2D {
    pub fn new(origin: Point2D, x_axis: Vector2D, y_axis: Vector2D) -> Self {
        Rectangle2D {
            origin,
            x_axis,
            y_axis,
        }
    }

    pub fn from_relative(&self, p: &Point2D) -> Point2D {
        self.origin + self.x_axis * p.x() + self.y_axis * p.y()
    }

    pub fn center(&self) -> Point2D {
        self.from_relative(&p2!(0.5, 0.5))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use crate::math::point2d::p2;
    use crate::math::vector2d::v2;
    use rstest::rstest;

    #[rstest]
    #[case(Rectangle2D::new(p2!(0, 0), v2!(1, 0), v2!(0, 1)), p2!(0, 0), p2!(0, 0))]
    #[case(Rectangle2D::new(p2!(0, 0), v2!(1, 0), v2!(0, 1)), p2!(1, 0), p2!(1, 0))]
    #[case(Rectangle2D::new(p2!(0, 0), v2!(1, 0), v2!(0, 1)), p2!(0, 1), p2!(0, 1))]
    #[case(Rectangle2D::new(p2!(0, 0), v2!(1, 0), v2!(0, 1)), p2!(1, 1), p2!(1, 1))]
    #[case(Rectangle2D::new(p2!(0, 0), v2!(2, 0), v2!(0, 4)), p2!(1, 0), p2!(2, 0))]
    #[case(Rectangle2D::new(p2!(0, 0), v2!(2, 0), v2!(0, 4)), p2!(0, 1), p2!(0, 4))]
    fn from_relative(
        #[case] rectangle: Rectangle2D,
        #[case] p: Point2D,
        #[case] expected: Point2D,
    ) {
        let actual = rectangle.from_relative(&p);

        assert_eq!(expected, actual);
    }
}
