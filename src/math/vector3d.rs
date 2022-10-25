#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    coords: [f64; 3],
}

#[macro_export]
macro_rules! v3 {
    ( $x:expr, $y:expr, $z:expr ) => {
        {
            Vector3D::new($x as f64, $y as f64, $z as f64)
        }
    };
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D {
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

impl std::ops::Add<Vector3D> for Vector3D {
    type Output = Self;

    fn add(self, v: Vector3D) -> Self::Output {
        let coords = [
            self.x() + v.x(),
            self.y() + v.y(),
            self.z() + v.z(),
        ];

        Vector3D { coords }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn addition() {
        let u = v3!(1, 2, 3);
        let v = v3!(5, 2, 4);
        let expected = v3!(6, 4, 7);
        let actual = u + v;

        assert_eq!(expected, actual);
    }
}
