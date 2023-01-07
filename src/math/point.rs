use super::{metric::Metric, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<const N: usize> {
    pub coords: [f64; N],
}

#[macro_export]
macro_rules! pt {
    ( $x:expr, $y:expr ) => {{
        $crate::math::Point::<2>::new([$x as f64, $y as f64])
    }};
    ( $x:expr, $y:expr, $z:expr ) => {{
        $crate::math::Point::<3>::new([$x as f64, $y as f64, $z as f64])
    }};
}

pub use pt;

impl<const N: usize> Point<N> {
    pub fn new(coords: [f64; N]) -> Self {
        Point { coords }
    }

    pub fn zero() -> Self {
        let coords = [0f64; N];

        Self::new(coords)
    }
}

impl Point<2> {
    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }
}

impl Point<3> {
    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn z(&self) -> f64 {
        self.coords[2]
    }
}

impl<const N: usize> std::ops::Add<Vector<N>> for Point<N> {
    type Output = Self;

    fn add(self, v: Vector<N>) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] + v.coords[i];
        }

        Point::new(result)
    }
}

impl<const N: usize> std::ops::Add<&Vector<N>> for &Point<N> {
    type Output = Point<N>;

    fn add(self, v: &Vector<N>) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] + v.coords[i];
        }

        Point::new(result)
    }
}

impl<const N: usize> std::ops::AddAssign<Vector<N>> for Point<N> {
    fn add_assign(&mut self, rhs: Vector<N>) {
        for i in 0..N {
            self.coords[i] += rhs.coords[i];
        }
    }
}

impl<const N: usize> std::ops::Sub<Point<N>> for Point<N> {
    type Output = Vector<N>;

    fn sub(self, p: Self) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] - p.coords[i];
        }

        Vector::new(result)
    }
}

impl<const N: usize> std::ops::Sub<&Point<N>> for &Point<N> {
    type Output = Vector<N>;

    fn sub(self, p: &Point<N>) -> Self::Output {
        let mut result = [0f64; N];

        for i in 0..N {
            result[i] = self.coords[i] - p.coords[i];
        }

        Vector::new(result)
    }
}

impl<const N: usize> Metric for Point<N> {
    fn distance(&self, rhs: &Self) -> f64 {
        (*self - *rhs).norm()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use crate::math::vector::vc;
    use rstest::rstest;

    #[rstest]
    #[case(pt!(0, 0), vc!(0, 0), pt!(0, 0))]
    #[case(pt!(1, 0), vc!(0, 0), pt!(1, 0))]
    #[case(pt!(0, 1), vc!(0, 0), pt!(0, 1))]
    #[case(pt!(0, 0), vc!(1, 0), pt!(1, 0))]
    #[case(pt!(0, 0), vc!(0, 1), pt!(0, 1))]
    #[case(pt!(1, 0), vc!(0, 0), pt!(1, 0))]
    #[case(pt!(1, 2), vc!(0, 0), pt!(1, 2))]
    #[case(pt!(1, 2), vc!(5, 2), pt!(6, 4))]
    fn addition2d(#[case] p: Point<2>, #[case] v: Vector<2>, #[case] expected: Point<2>) {
        let actual = p + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(pt!(0, 0), pt!(0, 0), vc!(0, 0))]
    #[case(pt!(1, 0), pt!(0, 0), vc!(1, 0))]
    #[case(pt!(0, 1), pt!(0, 0), vc!(0, 1))]
    #[case(pt!(0, 0), pt!(0, 0), vc!(0, 0))]
    #[case(pt!(0, 0), pt!(1, 0), vc!(-1, 0))]
    #[case(pt!(0, 0), pt!(0, 1), vc!(0, -1))]
    #[case(pt!(0, 0), pt!(0, 0), vc!(0, 0))]
    #[case(pt!(1, 0), pt!(0, 0), vc!(1, 0))]
    #[case(pt!(1, 2), pt!(0, 0), vc!(1, 2))]
    #[case(pt!(5, 2), pt!(1, 2), vc!(4, 0))]
    fn subtraction_p_p2d(#[case] p: Point<2>, #[case] q: Point<2>, #[case] expected: Vector<2>) {
        let actual = p - q;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(pt!(0, 0), pt!(0, 0), 0.0)]
    #[case(pt!(1, 0), pt!(0, 0), 1.0)]
    #[case(pt!(0, 1), pt!(0, 0), 1.0)]
    #[case(pt!(0, 0), pt!(1, 0), 1.0)]
    #[case(pt!(0, 0), pt!(0, 1), 1.0)]
    #[case(pt!(2, 0), pt!(0, 0), 2.0)]
    #[case(pt!(3, 4), pt!(0, 0), 5.0)]
    #[case(pt!(4, 5), pt!(1, 1), 5.0)]
    fn distance2d(#[case] u: Point<2>, #[case] v: Point<2>, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(pt!(0, 0, 0), vc!(0, 0, 0), pt!(0, 0, 0))]
    #[case(pt!(1, 0, 0), vc!(0, 0, 0), pt!(1, 0, 0))]
    #[case(pt!(0, 1, 0), vc!(0, 0, 0), pt!(0, 1, 0))]
    #[case(pt!(0, 0, 1), vc!(0, 0, 0), pt!(0, 0, 1))]
    #[case(pt!(0, 0, 0), vc!(1, 0, 0), pt!(1, 0, 0))]
    #[case(pt!(0, 0, 0), vc!(0, 1, 0), pt!(0, 1, 0))]
    #[case(pt!(0, 0, 0), vc!(0, 0, 1), pt!(0, 0, 1))]
    #[case(pt!(1, 0, 0), vc!(0, 0, 1), pt!(1, 0, 1))]
    #[case(pt!(1, 2, 3), vc!(0, 0, 0), pt!(1, 2, 3))]
    #[case(pt!(1, 2, 3), vc!(5, 2, 4), pt!(6, 4, 7))]
    fn addition3d(#[case] p: Point<3>, #[case] v: Vector<3>, #[case] expected: Point<3>) {
        let actual = p + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(pt!(0, 0, 0), pt!(0, 0, 0), vc!(0, 0, 0))]
    #[case(pt!(1, 0, 0), pt!(0, 0, 0), vc!(1, 0, 0))]
    #[case(pt!(0, 1, 0), pt!(0, 0, 0), vc!(0, 1, 0))]
    #[case(pt!(0, 0, 1), pt!(0, 0, 0), vc!(0, 0, 1))]
    #[case(pt!(0, 0, 0), pt!(1, 0, 0), vc!(-1, 0, 0))]
    #[case(pt!(0, 0, 0), pt!(0, 1, 0), vc!(0, -1, 0))]
    #[case(pt!(0, 0, 0), pt!(0, 0, 1), vc!(0, 0, -1))]
    #[case(pt!(1, 0, 0), pt!(0, 0, 1), vc!(1, 0, -1))]
    #[case(pt!(1, 2, 3), pt!(0, 0, 0), vc!(1, 2, 3))]
    #[case(pt!(5, 2, 4), pt!(1, 2, 3), vc!(4, 0, 1))]
    fn subtraction_p_p3d(#[case] p: Point<3>, #[case] q: Point<3>, #[case] expected: Vector<3>) {
        let actual = p - q;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(pt!(0, 0, 0), pt!(0, 0, 0), 0.0)]
    #[case(pt!(1, 0, 0), pt!(0, 0, 0), 1.0)]
    #[case(pt!(0, 1, 0), pt!(0, 0, 0), 1.0)]
    #[case(pt!(0, 0, 1), pt!(0, 0, 0), 1.0)]
    #[case(pt!(0, 0, 0), pt!(1, 0, 0), 1.0)]
    #[case(pt!(0, 0, 0), pt!(0, 1, 0), 1.0)]
    #[case(pt!(0, 0, 0), pt!(0, 0, 1), 1.0)]
    #[case(pt!(2, 0, 0), pt!(0, 0, 0), 2.0)]
    #[case(pt!(3, 4, 0), pt!(0, 0, 0), 5.0)]
    #[case(pt!(4, 5, 0), pt!(1, 1, 0), 5.0)]
    #[case(pt!(4, 0, 5), pt!(1, 0, 1), 5.0)]
    #[case(pt!(4, 0, 1), pt!(1, 0, 5), 5.0)]
    fn distance3d(#[case] u: Point<3>, #[case] v: Point<3>, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }
}
