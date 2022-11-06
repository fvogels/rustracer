use crate::math::{approx::approx, metric::Metric};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    coords: [f64; 2],
}

#[macro_export]
macro_rules! v2 {
    ( $x:expr, $y:expr ) => {{
        $crate::math::vector2d::Vector2D::new($x as f64, $y as f64)
    }};
}

pub use v2;

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { coords: [x, y] }
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn dot(&self, v: &Vector2D) -> f64 {
        let x = self.x() * v.x();
        let y = self.y() * v.y();

        x + y
    }

    pub fn norm_sqr(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }

    pub fn is_orthogonal_to(&self, v: &Vector2D) -> bool {
        approx(0.0) == self.dot(v)
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;

    fn add(self, v: Vector2D) -> Self::Output {
        let x = self.x() + v.x();
        let y = self.y() + v.y();

        Vector2D::new(x, y)
    }
}

impl std::ops::Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        let x = -self.x();
        let y = -self.y();

        v2!(x, y)
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, v: Vector2D) -> Self::Output {
        let x = self.x() - v.x();
        let y = self.y() - v.y();

        Vector2D::new(x, y)
    }
}

impl std::ops::Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x() * rhs;
        let y = self.y() * rhs;

        Vector2D::new(x, y)
    }
}

impl Metric for Vector2D {
    fn distance(&self, rhs: &Self) -> f64 {
        (*self - *rhs).norm()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(v2!(0, 0), v2!(0, 0), v2!(0, 0))]
    #[case(v2!(1, 0), v2!(0, 0), v2!(1, 0))]
    #[case(v2!(0, 1), v2!(0, 0), v2!(0, 1))]
    #[case(v2!(0, 0), v2!(1, 0), v2!(1, 0))]
    #[case(v2!(0, 0), v2!(0, 1), v2!(0, 1))]
    #[case(v2!(1, 0), v2!(0, 0), v2!(1, 0))]
    #[case(v2!(1, 2), v2!(0, 0), v2!(1, 2))]
    #[case(v2!(1, 2), v2!(5, 2), v2!(6, 4))]
    fn addition(#[case] u: Vector2D, #[case] v: Vector2D, #[case] expected: Vector2D) {
        let actual = u + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v2!(0, 0), v2!(0, 0), 0.0)]
    #[case(v2!(1, 0), v2!(0, 0), 0.0)]
    #[case(v2!(1, 0), v2!(1, 0), 1.0)]
    #[case(v2!(2, 0), v2!(3, 0), 6.0)]
    #[case(v2!(0, 4), v2!(0, 5), 20.0)]
    #[case(v2!(1, 2), v2!(1, 2), 1.0 + 4.0)]
    fn dot_product(#[case] u: Vector2D, #[case] v: Vector2D, #[case] expected: f64) {
        let actual = u.dot(&v);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v2!(0, 0), 0.0, v2!(0, 0))]
    #[case(v2!(1, 1), 0.0, v2!(0, 0))]
    #[case(v2!(0, 0), 1.0, v2!(0, 0))]
    #[case(v2!(1, 0), 1.0, v2!(1, 0))]
    #[case(v2!(1, 0), 2.0, v2!(2, 0))]
    #[case(v2!(3, 0), 2.0, v2!(6, 0))]
    #[case(v2!(0, 4), 2.0, v2!(0, 8))]
    #[case(v2!(1, 2), 2.0, v2!(2, 4))]
    fn multiplication(#[case] v: Vector2D, #[case] c: f64, #[case] expected: Vector2D) {
        let actual = v * c;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v2!(0, 0), 0.0)]
    #[case(v2!(1, 0), 1.0)]
    #[case(v2!(0, 1), 1.0)]
    #[case(v2!(3, 4), 5.0)]
    #[case(v2!(3, -4), 5.0)]
    #[case(v2!(-3, -4), 5.0)]
    #[case(v2!(-3, 4), 5.0)]
    fn norm(#[case] v: Vector2D, #[case] expected: f64) {
        let actual = v.norm();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v2!(1, 0), v2!(0, 1), true)]
    #[case(v2!(1, 0), v2!(0, 2), true)]
    #[case(v2!(1, 0), v2!(0, 5), true)]
    #[case(v2!(2, 0), v2!(0, 5), true)]
    #[case(v2!(4, 0), v2!(0, 5), true)]
    #[case(v2!(4, 1), v2!(0, 5), false)]
    fn is_orthogonal_to(#[case] u: Vector2D, #[case] v: Vector2D, #[case] expected: bool) {
        assert_eq!(expected, u.is_orthogonal_to(&v));
        assert_eq!(expected, v.is_orthogonal_to(&u));
    }

    #[rstest]
    #[case(v2!(0, 0), v2!(0, 0), 0.0)]
    #[case(v2!(1, 0), v2!(0, 0), 1.0)]
    #[case(v2!(0, 1), v2!(0, 0), 1.0)]
    #[case(v2!(0, 0), v2!(1, 0), 1.0)]
    #[case(v2!(0, 0), v2!(0, 1), 1.0)]
    #[case(v2!(2, 0), v2!(0, 0), 2.0)]
    #[case(v2!(3, 4), v2!(0, 0), 5.0)]
    #[case(v2!(4, 5), v2!(1, 1), 5.0)]
    fn distance(#[case] u: Vector2D, #[case] v: Vector2D, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }
}
