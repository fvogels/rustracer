use std::ops::Mul;

use super::{point3d::{Point3D, p3}, vector3d::Vector3D, ray::Ray};

pub struct Matrix4D {
    m: [[f64; 4]; 4],
}

impl Matrix4D {
    fn zero() -> Matrix4D {
        let m = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];

        Matrix4D { m }
    }

    fn identity() -> Matrix4D {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Matrix4D { m }
    }

    fn from_coordinate_system(origin: Point3D, x_axis: Vector3D, y_axis: Vector3D, z_axis: Vector3D) -> Matrix4D {
        let m = [
            [x_axis.x(), y_axis.x(), z_axis.x(), origin.x()],
            [x_axis.y(), y_axis.y(), z_axis.y(), origin.y()],
            [x_axis.z(), y_axis.z(), z_axis.z(), origin.z()],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Matrix4D { m }
    }

    fn translation(v: Vector3D) -> Matrix4D {
        let origin = p3!(v.x(), v.y(), v.z());
        let x_axis = Vector3D::x_axis();
        let y_axis = Vector3D::y_axis();
        let z_axis = Vector3D::z_axis();
        Matrix4D::from_coordinate_system(origin, x_axis, y_axis, z_axis)
    }

    fn scaling(sx: f64, sy: f64, sz: f64) -> Matrix4D {
        let origin = p3!(0, 0, 0);
        let x_axis = Vector3D::x_axis() * sx;
        let y_axis = Vector3D::y_axis() * sy;
        let z_axis = Vector3D::z_axis() * sz;
        Matrix4D::from_coordinate_system(origin, x_axis, y_axis, z_axis)
    }
}

impl Mul<&Matrix4D> for &Matrix4D {
    type Output = Matrix4D;

    fn mul(self, rhs: &Matrix4D) -> Self::Output {
        let lhs = self;
        let mut result = Matrix4D::zero();

        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    result.m[row][col] += lhs.m[row][i] * rhs.m[i][col];
                }
            }
        }

        result
    }
}

impl Mul<&Vector3D> for &Matrix4D {
    type Output = Vector3D;

    fn mul(self, v: &Vector3D) -> Self::Output {
        let m = &self.m;
        let x = m[0][0] * v.x() + m[0][1] * v.y() + m[0][2] * v.z();
        let y = m[1][0] * v.x() + m[1][1] * v.y() + m[1][2] * v.z();
        let z = m[2][0] * v.x() + m[2][1] * v.y() + m[2][2] * v.z();

        Vector3D::new(x, y, z)
    }
}

impl Mul<&Point3D> for &Matrix4D {
    type Output = Point3D;

    fn mul(self, v: &Point3D) -> Self::Output {
        let m = &self.m;
        let x = m[0][0] * v.x() + m[0][1] * v.y() + m[0][2] * v.z() + m[0][3];
        let y = m[1][0] * v.x() + m[1][1] * v.y() + m[1][2] * v.z() + m[1][3];
        let z = m[2][0] * v.x() + m[2][1] * v.y() + m[2][2] * v.z() + m[2][3];

        Point3D::new(x, y, z)
    }
}

impl Mul<&Ray> for &Matrix4D {
    type Output = Ray;

    fn mul(self, ray: &Ray) -> Self::Output {
        let origin = self * &ray.origin;
        let direction = self * &ray.direction;

        Ray::new(origin, direction)
    }
}

mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::math::{vector3d::v3, point3d::p3};

    #[rstest]
    #[case(v3!(0, 0, 0), v3!(0, 0, 0), v3!(0, 0, 0))]
    #[case(v3!(0, 0, 0), v3!(1, 0, 0), v3!(1, 0, 0))]
    #[case(v3!(1, 0, 0), v3!(1, 0, 0), v3!(1, 0, 0))]
    #[case(v3!(1, 0, 0), v3!(0, 1, 0), v3!(0, 1, 0))]
    #[case(v3!(1, 0, 0), v3!(0, 0, 1), v3!(0, 0, 1))]
    #[case(v3!(1, 2, 0), v3!(1, 0, 0), v3!(1, 0, 0))]
    #[case(v3!(1, 2, 3), v3!(1, 0, 0), v3!(1, 0, 0))]
    #[case(v3!(1, 4, 2), v3!(1, 2, 3), v3!(1, 2, 3))]
    fn translate_vector(#[case] displacement: Vector3D, #[case] v: Vector3D, #[case] expected: Vector3D) {
        let matrix = Matrix4D::translation(displacement);
        let actual = &matrix * &v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(v3!(0, 0, 0), p3!(0, 0, 0), p3!(0, 0, 0))]
    #[case(v3!(0, 0, 0), p3!(1, 0, 0), p3!(1, 0, 0))]
    #[case(v3!(1, 0, 0), p3!(0, 0, 0), p3!(1, 0, 0))]
    #[case(v3!(1, 0, 0), p3!(1, 0, 0), p3!(2, 0, 0))]
    #[case(v3!(1, 0, 0), p3!(0, 1, 0), p3!(1, 1, 0))]
    #[case(v3!(1, 0, 0), p3!(0, 0, 1), p3!(1, 0, 1))]
    #[case(v3!(1, 2, 0), p3!(1, 0, 0), p3!(2, 2, 0))]
    #[case(v3!(1, 2, 3), p3!(1, 0, 0), p3!(2, 2, 3))]
    #[case(v3!(1, 4, 2), p3!(5, 1, 7), p3!(6, 5, 9))]
    fn translate_point(#[case] displacement: Vector3D, #[case] p: Point3D, #[case] expected: Point3D) {
        let matrix = Matrix4D::translation(displacement);
        let actual = &matrix * &p;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, v3!(1, 1, 1), v3!(0, 0, 0))]
    #[case(1.0, 1.0, 1.0, v3!(1, 1, 1), v3!(1, 1, 1))]
    #[case(2.0, 1.0, 1.0, v3!(1, 1, 1), v3!(2, 1, 1))]
    #[case(1.0, 2.0, 1.0, v3!(1, 1, 1), v3!(1, 2, 1))]
    #[case(1.0, 1.0, 2.0, v3!(1, 1, 1), v3!(1, 1, 2))]
    #[case(2.0, 3.0, 2.0, v3!(2, 3, 4), v3!(4, 9, 8))]
    fn scale_vector(#[case] sx: f64, #[case] sy: f64, #[case] sz: f64, #[case] v: Vector3D, #[case] expected: Vector3D) {
        let matrix = Matrix4D::scaling(sx, sy, sz);
        let actual = &matrix * &v;

        assert_eq!(expected, actual);
    }
}
