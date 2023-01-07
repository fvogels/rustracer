use crate::math::{approx::approx, metric::Metric};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    pub coords: [f64; N],
}

#[macro_export]
macro_rules! vc {
    ( $x:expr, $y:expr ) => {{
        $crate::math::Vector::<2>::new([$x as f64, $y as f64])
    }};
    ( $x:expr, $y:expr, $z:expr ) => {{
        $crate::math::Vector::<3>::new([$x as f64, $y as f64, $z as f64])
    }};
}

pub use vc;

impl<const N: usize> Vector<N> {
    pub fn new(coords: [f64; N]) -> Self {
        Vector { coords }
    }

    pub fn dot(&self, v: &Self) -> f64 {
        let mut result = 0f64;

        for i in 0..N {
            result += self.coords[i] * v.coords[i];
        }

        result
    }

    pub fn norm_sqr(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self * self.norm().recip()
    }

    pub fn is_orthogonal_to(&self, v: &Self) -> bool {
        approx(0.0) == self.dot(v)
    }

    pub fn is_unit(&self) -> bool {
        approx(1.0) == self.norm_sqr()
    }

    pub fn cos_angle_between(&self, v: &Self) -> f64 {
        self.normalized().dot(&v.normalized())
    }
}

impl Vector<2> {
    pub fn x_axis() -> Self {
        vc!(1, 0)
    }

    pub fn y_axis() -> Self {
        vc!(0, 1)
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }
}

impl Vector<3> {
    pub fn x_axis() -> Self {
        vc!(1, 0, 0)
    }

    pub fn y_axis() -> Self {
        vc!(0, 1, 0)
    }

    pub fn z_axis() -> Self {
        vc!(0, 0, 1)
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

    pub fn cross(&self, v: &Self) -> Self {
        let u = self;

        let x = u.y() * v.z() - u.z() * v.y();
        let y = -(u.x() * v.z() - u.z() * v.x());
        let z = u.x() * v.y() - u.y() * v.x();

        vc!(x, y, z)
    }
}

impl<const N: usize> std::ops::Add for Vector<N> {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] + v.coords[i];
        }

        Vector::new(result)
    }
}

impl<const N: usize> std::ops::Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = -self.coords[i];
        }

        Vector::new(result)
    }
}

impl<const N: usize> std::ops::Neg for &Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = -self.coords[i];
        }

        Vector::new(result)
    }
}

impl<const N: usize> std::ops::Sub for Vector<N> {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] - v.coords[i];
        }

        Vector::new(result)
    }
}

impl<const N: usize> std::ops::Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] * rhs;
        }

        Vector::new(result)
    }
}

impl<const N: usize> Metric for Vector<N> {
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
    #[case(vc!(0, 0), vc!(0, 0), vc!(0, 0))]
    #[case(vc!(1, 0), vc!(0, 0), vc!(1, 0))]
    #[case(vc!(0, 1), vc!(0, 0), vc!(0, 1))]
    #[case(vc!(0, 0), vc!(1, 0), vc!(1, 0))]
    #[case(vc!(0, 0), vc!(0, 1), vc!(0, 1))]
    #[case(vc!(1, 0), vc!(0, 0), vc!(1, 0))]
    #[case(vc!(1, 2), vc!(0, 0), vc!(1, 2))]
    #[case(vc!(1, 2), vc!(5, 2), vc!(6, 4))]
    fn addition(#[case] u: Vector<2>, #[case] v: Vector<2>, #[case] expected: Vector<2>) {
        let actual = u + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(vc!(0, 0), vc!(0, 0), 0.0)]
    #[case(vc!(1, 0), vc!(0, 0), 0.0)]
    #[case(vc!(1, 0), vc!(1, 0), 1.0)]
    #[case(vc!(2, 0), vc!(3, 0), 6.0)]
    #[case(vc!(0, 4), vc!(0, 5), 20.0)]
    #[case(vc!(1, 2), vc!(1, 2), 1.0 + 4.0)]
    fn dot_product(#[case] u: Vector<2>, #[case] v: Vector<2>, #[case] expected: f64) {
        let actual = u.dot(&v);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(vc!(0, 0), 0.0, vc!(0, 0))]
    #[case(vc!(1, 1), 0.0, vc!(0, 0))]
    #[case(vc!(0, 0), 1.0, vc!(0, 0))]
    #[case(vc!(1, 0), 1.0, vc!(1, 0))]
    #[case(vc!(1, 0), 2.0, vc!(2, 0))]
    #[case(vc!(3, 0), 2.0, vc!(6, 0))]
    #[case(vc!(0, 4), 2.0, vc!(0, 8))]
    #[case(vc!(1, 2), 2.0, vc!(2, 4))]
    fn multiplication(#[case] v: Vector<2>, #[case] c: f64, #[case] expected: Vector<2>) {
        let actual = v * c;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(vc!(0, 0), 0.0)]
    #[case(vc!(1, 0), 1.0)]
    #[case(vc!(0, 1), 1.0)]
    #[case(vc!(3, 4), 5.0)]
    #[case(vc!(3, -4), 5.0)]
    #[case(vc!(-3, -4), 5.0)]
    #[case(vc!(-3, 4), 5.0)]
    fn norm(#[case] v: Vector<2>, #[case] expected: f64) {
        let actual = v.norm();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(vc!(1, 0), vc!(0, 1), true)]
    #[case(vc!(1, 0), vc!(0, 2), true)]
    #[case(vc!(1, 0), vc!(0, 5), true)]
    #[case(vc!(2, 0), vc!(0, 5), true)]
    #[case(vc!(4, 0), vc!(0, 5), true)]
    #[case(vc!(4, 1), vc!(0, 5), false)]
    fn is_orthogonal_to(#[case] u: Vector<2>, #[case] v: Vector<2>, #[case] expected: bool) {
        assert_eq!(expected, u.is_orthogonal_to(&v));
        assert_eq!(expected, v.is_orthogonal_to(&u));
    }

    #[rstest]
    #[case(vc!(0, 0), vc!(0, 0), 0.0)]
    #[case(vc!(1, 0), vc!(0, 0), 1.0)]
    #[case(vc!(0, 1), vc!(0, 0), 1.0)]
    #[case(vc!(0, 0), vc!(1, 0), 1.0)]
    #[case(vc!(0, 0), vc!(0, 1), 1.0)]
    #[case(vc!(2, 0), vc!(0, 0), 2.0)]
    #[case(vc!(3, 4), vc!(0, 0), 5.0)]
    #[case(vc!(4, 5), vc!(1, 1), 5.0)]
    fn distance(#[case] u: Vector<2>, #[case] v: Vector<2>, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }
}
