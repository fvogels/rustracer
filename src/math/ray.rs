use super::point3d::Point3D;
use super::vector3d::Vector3D;


pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::{Point3D, Vector3D, Ray};
    use crate::math::point3d::p3;
    use crate::math::vector3d::v3;
    use rstest::rstest;

    #[rstest]
    #[case(p3!(0, 0, 0), v3!(1, 0, 0), 1.0, p3!(1, 0, 0))]
    #[case(p3!(0, 0, 0), v3!(1, 0, 0), 2.0, p3!(2, 0, 0))]
    #[case(p3!(1, 0, 0), v3!(1, 0, 0), 1.0, p3!(2, 0, 0))]
    #[case(p3!(1, 0, 0), v3!(2, 0, 0), 1.0, p3!(3, 0, 0))]
    #[case(p3!(1, 2, 3), v3!(1, 1, 1), 1.0, p3!(2, 3, 4))]
    #[case(p3!(1, 2, 3), v3!(1, 1, 1), 2.0, p3!(3, 4, 5))]
    fn at(#[case] position: Point3D, #[case] direction: Vector3D, #[case] t: f64, #[case] expected: Point3D) {
        let actual = Ray::new(position, direction).at(t);

        assert_eq!(expected, actual);
    }
}