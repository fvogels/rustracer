use crate::math::vector3d::Vector3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    coords: [f64; 3],
}

#[macro_export]
macro_rules! p3 {
    ( $x:expr, $y:expr, $z:expr ) => {
        {
            Point3D::new($x as f64, $y as f64, $z as f64)
        }
    };
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D {
            coords: [x, y, z]
        }
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
        let coords = [
            self.x() + v.x(),
            self.y() + v.y(),
            self.z() + v.z(),
        ];

        Point3D { coords }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
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
}
