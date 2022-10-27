#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    coords: [f64; 3],
}

#[macro_export]
macro_rules! v3 {
    ( $x:expr, $y:expr, $z:expr ) => {{
        Vector3D::new($x as f64, $y as f64, $z as f64)
    }};
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { coords: [x, y, z] }
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

    pub fn dot(&self, v: Vector3D) -> f64 {
        let x = self.x() * v.x();
        let y = self.y() * v.y();
        let z = self.z() * v.z();

        x + y + z
    }
}

impl std::ops::Add for Vector3D {
    type Output = Self;

    fn add(self, v: Vector3D) -> Self::Output {
        let coords = [self.x() + v.x(), self.y() + v.y(), self.z() + v.z()];

        Vector3D { coords }
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
        let actual = u.dot(v);

        assert_eq!(expected, actual);
    }
}
