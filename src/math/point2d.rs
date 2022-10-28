use super::{metric::Metric, vector2d::Vector2D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    coords: [f64; 2],
}

#[macro_export]
macro_rules! p2 {
    ( $x:expr, $y:expr ) => {{
        Point2D::new($x as f64, $y as f64)
    }};
}

pub use p2;

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { coords: [x, y] }
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }
}

impl std::ops::Add<Vector2D> for Point2D {
    type Output = Self;

    fn add(self, v: Vector2D) -> Self::Output {
        let x = self.x() + v.x();
        let y = self.y() + v.y();

        Point2D::new(x, y)
    }
}

impl std::ops::Sub<Point2D> for Point2D {
    type Output = Vector2D;

    fn sub(self, p: Point2D) -> Self::Output {
        let x = self.x() - p.x();
        let y = self.y() - p.y();

        Vector2D::new(x, y)
    }
}

impl Metric for Point2D {
    fn distance(&self, rhs: &Self) -> f64 {
        (*self - *rhs).norm()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use crate::math::vector2d::v2;
    use rstest::rstest;

    #[rstest]
    #[case(p2!(0, 0), v2!(0, 0), p2!(0, 0))]
    #[case(p2!(1, 0), v2!(0, 0), p2!(1, 0))]
    #[case(p2!(0, 1), v2!(0, 0), p2!(0, 1))]
    #[case(p2!(0, 0), v2!(1, 0), p2!(1, 0))]
    #[case(p2!(0, 0), v2!(0, 1), p2!(0, 1))]
    #[case(p2!(1, 0), v2!(0, 0), p2!(1, 0))]
    #[case(p2!(1, 2), v2!(0, 0), p2!(1, 2))]
    #[case(p2!(1, 2), v2!(5, 2), p2!(6, 4))]
    fn addition(#[case] p: Point2D, #[case] v: Vector2D, #[case] expected: Point2D) {
        let actual = p + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(p2!(0, 0), p2!(0, 0), v2!(0, 0))]
    #[case(p2!(1, 0), p2!(0, 0), v2!(1, 0))]
    #[case(p2!(0, 1), p2!(0, 0), v2!(0, 1))]
    #[case(p2!(0, 0), p2!(0, 0), v2!(0, 0))]
    #[case(p2!(0, 0), p2!(1, 0), v2!(-1, 0))]
    #[case(p2!(0, 0), p2!(0, 1), v2!(0, -1))]
    #[case(p2!(0, 0), p2!(0, 0), v2!(0, 0))]
    #[case(p2!(1, 0), p2!(0, 0), v2!(1, 0))]
    #[case(p2!(1, 2), p2!(0, 0), v2!(1, 2))]
    #[case(p2!(5, 2), p2!(1, 2), v2!(4, 0))]
    fn subtraction_p_p(#[case] p: Point2D, #[case] q: Point2D, #[case] expected: Vector2D) {
        let actual = p - q;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(p2!(0, 0), p2!(0, 0), 0.0)]
    #[case(p2!(1, 0), p2!(0, 0), 1.0)]
    #[case(p2!(0, 1), p2!(0, 0), 1.0)]
    #[case(p2!(0, 0), p2!(1, 0), 1.0)]
    #[case(p2!(0, 0), p2!(0, 1), 1.0)]
    #[case(p2!(2, 0), p2!(0, 0), 2.0)]
    #[case(p2!(3, 4), p2!(0, 0), 5.0)]
    #[case(p2!(4, 5), p2!(1, 1), 5.0)]
    fn distance(#[case] u: Point2D, #[case] v: Point2D, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }
}
