use super::{metric::Metric, vector3d::Vector3D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    coords: [f64; 3],
}

#[macro_export]
macro_rules! p3 {
    ( $x:expr, $y:expr, $z:expr ) => {{
        $crate::math::point3d::Point3D::new($x as f64, $y as f64, $z as f64)
    }};
}

pub use p3;

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { coords: [x, y, z] }
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
}

impl std::ops::Add<Vector3D> for Point3D {
    type Output = Self;

    fn add(self, v: Vector3D) -> Self::Output {
        let x = self.x() + v.x();
        let y = self.y() + v.y();
        let z = self.z() + v.z();

        Point3D::new(x, y, z)
    }
}

impl std::ops::Sub<Point3D> for Point3D {
    type Output = Vector3D;

    fn sub(self, p: Point3D) -> Self::Output {
        let x = self.x() - p.x();
        let y = self.y() - p.y();
        let z = self.z() - p.z();

        Vector3D::new(x, y, z)
    }
}

impl Metric for Point3D {
    fn distance(&self, rhs: &Self) -> f64 {
        (*self - *rhs).norm()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use crate::math::vector3d::v3;
    use rstest::rstest;

    #[rstest]
    #[case(p3!(0, 0, 0), v3!(0, 0, 0), p3!(0, 0, 0))]
    #[case(p3!(1, 0, 0), v3!(0, 0, 0), p3!(1, 0, 0))]
    #[case(p3!(0, 1, 0), v3!(0, 0, 0), p3!(0, 1, 0))]
    #[case(p3!(0, 0, 1), v3!(0, 0, 0), p3!(0, 0, 1))]
    #[case(p3!(0, 0, 0), v3!(1, 0, 0), p3!(1, 0, 0))]
    #[case(p3!(0, 0, 0), v3!(0, 1, 0), p3!(0, 1, 0))]
    #[case(p3!(0, 0, 0), v3!(0, 0, 1), p3!(0, 0, 1))]
    #[case(p3!(1, 0, 0), v3!(0, 0, 1), p3!(1, 0, 1))]
    #[case(p3!(1, 2, 3), v3!(0, 0, 0), p3!(1, 2, 3))]
    #[case(p3!(1, 2, 3), v3!(5, 2, 4), p3!(6, 4, 7))]
    fn addition(#[case] p: Point3D, #[case] v: Vector3D, #[case] expected: Point3D) {
        let actual = p + v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(p3!(0, 0, 0), p3!(0, 0, 0), v3!(0, 0, 0))]
    #[case(p3!(1, 0, 0), p3!(0, 0, 0), v3!(1, 0, 0))]
    #[case(p3!(0, 1, 0), p3!(0, 0, 0), v3!(0, 1, 0))]
    #[case(p3!(0, 0, 1), p3!(0, 0, 0), v3!(0, 0, 1))]
    #[case(p3!(0, 0, 0), p3!(1, 0, 0), v3!(-1, 0, 0))]
    #[case(p3!(0, 0, 0), p3!(0, 1, 0), v3!(0, -1, 0))]
    #[case(p3!(0, 0, 0), p3!(0, 0, 1), v3!(0, 0, -1))]
    #[case(p3!(1, 0, 0), p3!(0, 0, 1), v3!(1, 0, -1))]
    #[case(p3!(1, 2, 3), p3!(0, 0, 0), v3!(1, 2, 3))]
    #[case(p3!(5, 2, 4), p3!(1, 2, 3), v3!(4, 0, 1))]
    fn subtraction_p_p(#[case] p: Point3D, #[case] q: Point3D, #[case] expected: Vector3D) {
        let actual = p - q;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(p3!(0, 0, 0), p3!(0, 0, 0), 0.0)]
    #[case(p3!(1, 0, 0), p3!(0, 0, 0), 1.0)]
    #[case(p3!(0, 1, 0), p3!(0, 0, 0), 1.0)]
    #[case(p3!(0, 0, 1), p3!(0, 0, 0), 1.0)]
    #[case(p3!(0, 0, 0), p3!(1, 0, 0), 1.0)]
    #[case(p3!(0, 0, 0), p3!(0, 1, 0), 1.0)]
    #[case(p3!(0, 0, 0), p3!(0, 0, 1), 1.0)]
    #[case(p3!(2, 0, 0), p3!(0, 0, 0), 2.0)]
    #[case(p3!(3, 4, 0), p3!(0, 0, 0), 5.0)]
    #[case(p3!(4, 5, 0), p3!(1, 1, 0), 5.0)]
    #[case(p3!(4, 0, 5), p3!(1, 0, 1), 5.0)]
    #[case(p3!(4, 0, 1), p3!(1, 0, 5), 5.0)]
    fn distance(#[case] u: Point3D, #[case] v: Point3D, #[case] expected: f64) {
        let actual = u.distance(&v);

        assert_eq!(expected, actual);
    }
}
