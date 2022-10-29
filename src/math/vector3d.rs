use crate::math::{approx::approx, metric::Metric};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    coords: [f64; 3],
}

#[macro_export]
macro_rules! v3 {
    ( $x:expr, $y:expr, $z:expr ) => {{
        $crate::math::vector3d::Vector3D::new($x as f64, $y as f64, $z as f64)
    }};
}

pub use v3;

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { coords: [x, y, z] }
    }

    pub fn x_axis() -> Vector3D {
        Vector3D::new(1.0, 0.0, 0.0)
    }

    pub fn y_axis() -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }

    pub fn z_axis() -> Vector3D {
        Vector3D::new(0.0, 0.0, 1.0)
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn z(&self) -> f64 {
        self.coords[2]
    }

    pub fn dot(&self, v: &Vector3D) -> f64 {
        let x = self.x() * v.x();
        let y = self.y() * v.y();
        let z = self.z() * v.z();

        x + y + z
    }

    pub fn cross(&self, v: &Vector3D) -> Vector3D {
        let u = self;

        let x = u.y() * v.z() - u.z() * v.y();
        let y = -(u.x() * v.z() - u.z() * v.x());
        let z = u.x() * v.y() - u.y() * v.x();

        v3!(x, y, z)
    }

    pub fn norm_sqr(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }

    pub fn normalized(&self) -> Vector3D {
        *self * self.norm().recip()
    }

    pub fn is_unit(&self) -> bool {
        approx(1.0) == self.norm_sqr()
    }

    pub fn is_orthogonal_to(&self, v: &Vector3D) -> bool {
        approx(0.0) == self.dot(v)
    }
}

impl std::ops::Add for Vector3D {
    type Output = Self;

    fn add(self, v: Vector3D) -> Self::Output {
        let x = self.x() + v.x();
        let y = self.y() + v.y();
        let z = self.z() + v.z();

        Vector3D::new(x, y, z)
    }
}

impl std::ops::Sub for Vector3D {
    type Output = Self;

    fn sub(self, v: Vector3D) -> Self::Output {
        let x = self.x() - v.x();
        let y = self.y() - v.y();
        let z = self.z() - v.z();

        Vector3D::new(x, y, z)
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x() * rhs;
        let y = self.y() * rhs;
        let z = self.z() * rhs;

        Vector3D::new(x, y, z)
    }
}

impl Metric for Vector3D {
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
    #[case(v3!(0, 0, 0), v3!(0, 0, 0), v3!(0, 0, 0))]
    #[case(v3!(1, 0, 0), v3!(0, 0, 0), v3!(1, 0, 0))]
    #[case(v3!(0, 1, 0), v3!(0, 0, 0), v3!(0, 1, 0))]
    #[case(v3!(0, 0, 1), v3!(0, 0, 0), v3!(0, 0, 1))]
    #[case(v3!(0, 0, 0), v3!(1, 0, 0), v3!(1, 0, 0))]
    #[case(v3!(0, 0, 0), v3!(0, 1, 0), v3!(0, 1, 0))]
    #[case(v3!(0, 0, 0), v3!(0, 0, 1), v3!(0, 0, 1))]
    #[case(v3!(1, 0, 0), v3!(0, 0, 1), v3!(1, 0, 1))]
    #[case(v3!(1, 2, 3), v3!(0, 0, 0), v3!(1, 2, 3))]
    #[case(v3!(1, 2, 3), v3!(5, 2, 4), v3!(6, 4, 7))]
    fn addition(#[case] u: Vector3D, #[case] v: Vector3D, #[case] expected: Vector3D) {
        let actual = u + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(0, 0, 0), v3!(0, 0, 0), 0.0)]
    #[case(v3!(1, 0, 0), v3!(0, 0, 0), 0.0)]
    #[case(v3!(1, 0, 0), v3!(1, 0, 0), 1.0)]
    #[case(v3!(2, 0, 0), v3!(3, 0, 0), 6.0)]
    #[case(v3!(0, 4, 0), v3!(0, 5, 0), 20.0)]
    #[case(v3!(0, 0, 2), v3!(0, 0, -1), -2.0)]
    #[case(v3!(1, 2, 3), v3!(1, 2, 3), 1.0 + 4.0 + 9.0)]
    fn dot_product(#[case] u: Vector3D, #[case] v: Vector3D, #[case] expected: f64) {
        let actual = u.dot(&v);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(0, 0, 0), 0.0, v3!(0, 0, 0))]
    #[case(v3!(1, 1, 1), 0.0, v3!(0, 0, 0))]
    #[case(v3!(0, 0, 0), 1.0, v3!(0, 0, 0))]
    #[case(v3!(1, 0, 0), 1.0, v3!(1, 0, 0))]
    #[case(v3!(1, 0, 0), 2.0, v3!(2, 0, 0))]
    #[case(v3!(3, 0, 0), 2.0, v3!(6, 0, 0))]
    #[case(v3!(0, 4, 0), 2.0, v3!(0, 8, 0))]
    #[case(v3!(0, 0, 2), 3.0, v3!(0, 0, 6))]
    #[case(v3!(1, 2, 3), 2.0, v3!(2, 4, 6))]
    fn multiplication(#[case] v: Vector3D, #[case] c: f64, #[case] expected: Vector3D) {
        let actual = v * c;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(0, 0, 0), 0.0)]
    #[case(v3!(1, 0, 0), 1.0)]
    #[case(v3!(0, 1, 0), 1.0)]
    #[case(v3!(0, 0, 1), 1.0)]
    #[case(v3!(3, 4, 0), 5.0)]
    #[case(v3!(3, -4, 0), 5.0)]
    #[case(v3!(-3, -4, 0), 5.0)]
    #[case(v3!(-3, 4, 0), 5.0)]
    #[case(v3!(-3, 0, 4), 5.0)]
    #[case(v3!(3, 0, 4), 5.0)]
    #[case(v3!(0, 3, 4), 5.0)]
    fn norm(#[case] v: Vector3D, #[case] expected: f64) {
        let actual = v.norm();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(1, 0, 0), v3!(0, 1, 0), true)]
    #[case(v3!(1, 0, 0), v3!(0, 0, 1), true)]
    #[case(v3!(1, 0, 0), v3!(0, 2, 0), true)]
    #[case(v3!(1, 0, 0), v3!(0, 0, 2), true)]
    #[case(v3!(0, 1, 0), v3!(0, 0, 2), true)]
    #[case(v3!(1, 0, 0), v3!(0, 5, 0), true)]
    #[case(v3!(2, 0, 0), v3!(0, 5, 0), true)]
    #[case(v3!(0, 0, 2), v3!(0, 5, 0), true)]
    #[case(v3!(4, 0, 0), v3!(0, 5, 0), true)]
    #[case(v3!(4, 1, 0), v3!(0, 5, 0), false)]
    #[case(v3!(4, 4, 0), v3!(4, 0, 4), false)]
    #[case(v3!(4, 4, 0), v3!(2, 0, 2), false)]
    fn is_orthogonal_to(#[case] u: Vector3D, #[case] v: Vector3D, #[case] expected: bool) {
        assert_eq!(expected, u.is_orthogonal_to(&v));
        assert_eq!(expected, v.is_orthogonal_to(&u));
    }

    #[rstest]
    #[case(v3!(1, 0, 0), v3!(0, 1, 0), v3!(0, 0, 1))]
    #[case(v3!(0, 1, 0), v3!(1, 0, 0), v3!(0, 0, -1))]
    #[case(v3!(0, 2, 0), v3!(1, 0, 0), v3!(0, 0, -2))]
    #[case(v3!(0, 2, 0), v3!(3, 0, 0), v3!(0, 0, -6))]
    #[case(v3!(1, 0, 0), v3!(0, 0, 1), v3!(0, -1, 0))]
    #[case(v3!(0, 0, 1), v3!(1, 0, 0), v3!(0, 1, 0))]
    fn cross_product(#[case] u: Vector3D, #[case] v: Vector3D, #[case] expected: Vector3D) {
        let actual = u.cross(&v);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(0, 0, 0), v3!(0, 0, 0), 0.0)]
    #[case(v3!(1, 0, 0), v3!(0, 0, 0), 1.0)]
    #[case(v3!(0, 1, 0), v3!(0, 0, 0), 1.0)]
    #[case(v3!(0, 0, 1), v3!(0, 0, 0), 1.0)]
    #[case(v3!(0, 0, 0), v3!(1, 0, 0), 1.0)]
    #[case(v3!(0, 0, 0), v3!(0, 1, 0), 1.0)]
    #[case(v3!(0, 0, 0), v3!(0, 0, 1), 1.0)]
    #[case(v3!(2, 0, 0), v3!(0, 0, 0), 2.0)]
    #[case(v3!(3, 4, 0), v3!(0, 0, 0), 5.0)]
    #[case(v3!(4, 5, 0), v3!(1, 1, 0), 5.0)]
    #[case(v3!(4, 0, 5), v3!(1, 0, 1), 5.0)]
    #[case(v3!(4, 0, 1), v3!(1, 0, 5), 5.0)]
    fn distance(#[case] u: Vector3D, #[case] v: Vector3D, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(0, 0, 0), false)]
    #[case(v3!(1, 0, 0), true)]
    #[case(v3!(0, 1, 0), true)]
    #[case(v3!(0, 0, 1), true)]
    #[case(v3!(2, 0, 0), false)]
    #[case(v3!(0, 2, 0), false)]
    #[case(v3!(0, 0, 2), false)]
    #[case(v3!(1, 0, 1), false)]
    fn is_unit(#[case] v: Vector3D, #[case] expected: bool) {
        let actual = v.is_unit();

        assert_eq!(expected, actual);
    }
}
