use std::ops::Mul;

use super::{
    angle::Angle,
    approx::Approx,
    point::{pt, Point},
    ray::Ray,
    vector::{vc, Vector},
};

#[derive(Debug)]
pub struct Matrix<const R: usize, const C: usize> {
    m: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zero() -> Self {
        let m = [[0.0; C]; R];

        Matrix { m }
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut result = Matrix::<N, N>::zero();

        for i in 0..N {
            result.m[i][i] = 1.0;
        }

        result
    }
}

impl Matrix<4, 4> {
    pub fn from_coordinate_system(
        origin: &Point<3>,
        x_axis: &Vector<3>,
        y_axis: &Vector<3>,
        z_axis: &Vector<3>,
    ) -> Self {
        let m = [
            [x_axis.x(), y_axis.x(), z_axis.x(), origin.x()],
            [x_axis.y(), y_axis.y(), z_axis.y(), origin.y()],
            [x_axis.z(), y_axis.z(), z_axis.z(), origin.z()],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Matrix { m }
    }

    pub fn translate(v: &Vector<3>) -> Self {
        let origin = pt!(v.x(), v.y(), v.z());
        let x_axis = Vector::<3>::x_axis();
        let y_axis = Vector::<3>::y_axis();
        let z_axis = Vector::<3>::z_axis();

        Matrix::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        let origin = pt!(0, 0, 0);
        let x_axis = Vector::<3>::x_axis() * sx;
        let y_axis = Vector::<3>::y_axis() * sy;
        let z_axis = Vector::<3>::z_axis() * sz;

        Matrix::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
    }

    pub fn rotate_around_x(angle: Angle) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        let origin = pt!(0, 0, 0);
        let x_axis = vc!(1, 0, 0);
        let y_axis = vc!(0, c, s);
        let z_axis = vc!(0, -s, c);

        Matrix::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
    }

    pub fn rotate_around_y(angle: Angle) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        let origin = pt!(0, 0, 0);
        let x_axis = vc!(c, 0, -s);
        let y_axis = vc!(0, 1, 0);
        let z_axis = vc!(s, 0, c);

        Matrix::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
    }

    pub fn rotate_around_z(angle: Angle) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        let origin = pt!(0, 0, 0);
        let x_axis = vc!(c, s, 0);
        let y_axis = vc!(-s, c, 0);
        let z_axis = vc!(0, 0, 1);

        Matrix::from_coordinate_system(&origin, &x_axis, &y_axis, &z_axis)
    }
}

impl<const A: usize, const B: usize, const C: usize> Mul<&Matrix<B, C>> for &Matrix<A, B> {
    type Output = Matrix<A, C>;

    fn mul(self, rhs: &Matrix<B, C>) -> Self::Output {
        let lhs = self;
        let mut result = Matrix::<A, C>::zero();

        for row in 0..A {
            for col in 0..C {
                for i in 0..B {
                    result.m[row][col] += lhs.m[row][i] * rhs.m[i][col];
                }
            }
        }

        result
    }
}

impl Mul<&Vector<3>> for &Matrix<4, 4> {
    type Output = Vector<3>;

    fn mul(self, v: &Vector<3>) -> Self::Output {
        let m = &self.m;
        let x = m[0][0] * v.x() + m[0][1] * v.y() + m[0][2] * v.z();
        let y = m[1][0] * v.x() + m[1][1] * v.y() + m[1][2] * v.z();
        let z = m[2][0] * v.x() + m[2][1] * v.y() + m[2][2] * v.z();

        vc!(x, y, z)
    }
}

impl Mul<&Point<3>> for &Matrix<4, 4> {
    type Output = Point<3>;

    fn mul(self, v: &Point<3>) -> Self::Output {
        let m = &self.m;
        let x = m[0][0] * v.x() + m[0][1] * v.y() + m[0][2] * v.z() + m[0][3];
        let y = m[1][0] * v.x() + m[1][1] * v.y() + m[1][2] * v.z() + m[1][3];
        let z = m[2][0] * v.x() + m[2][1] * v.y() + m[2][2] * v.z() + m[2][3];

        pt!(x, y, z)
    }
}

impl Mul<&Ray> for &Matrix<4, 4> {
    type Output = Ray;

    fn mul(self, ray: &Ray) -> Self::Output {
        let origin = self * &ray.origin;
        let direction = self * &ray.direction;

        Ray::new(origin, direction)
    }
}

impl<const R: usize, const C: usize> Approx for Matrix<R, C> {
    fn approx_eps(&self, rhs: &Self, epsilon: f64) -> bool {
        (0..R).all(|row| {
            (0..C).all(|col| {
                let left = self.m[row][col];
                let right = rhs.m[row][col];
                left.approx_eps(&right, epsilon)
            })
        })
    }
}

mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::math::approx::approx;

    #[rstest]
    #[case(vc!(0, 0, 0), vc!(0, 0, 0), vc!(0, 0, 0))]
    #[case(vc!(0, 0, 0), vc!(1, 0, 0), vc!(1, 0, 0))]
    #[case(vc!(1, 0, 0), vc!(1, 0, 0), vc!(1, 0, 0))]
    #[case(vc!(1, 0, 0), vc!(0, 1, 0), vc!(0, 1, 0))]
    #[case(vc!(1, 0, 0), vc!(0, 0, 1), vc!(0, 0, 1))]
    #[case(vc!(1, 2, 0), vc!(1, 0, 0), vc!(1, 0, 0))]
    #[case(vc!(1, 2, 3), vc!(1, 0, 0), vc!(1, 0, 0))]
    #[case(vc!(1, 4, 2), vc!(1, 2, 3), vc!(1, 2, 3))]
    fn translate_vector(
        #[case] displacement: Vector<3>,
        #[case] v: Vector<3>,
        #[case] expected: Vector<3>,
    ) {
        let matrix = Matrix::translate(&displacement);
        let actual = &matrix * &v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(vc!(0, 0, 0), pt!(0, 0, 0), pt!(0, 0, 0))]
    #[case(vc!(0, 0, 0), pt!(1, 0, 0), pt!(1, 0, 0))]
    #[case(vc!(1, 0, 0), pt!(0, 0, 0), pt!(1, 0, 0))]
    #[case(vc!(1, 0, 0), pt!(1, 0, 0), pt!(2, 0, 0))]
    #[case(vc!(1, 0, 0), pt!(0, 1, 0), pt!(1, 1, 0))]
    #[case(vc!(1, 0, 0), pt!(0, 0, 1), pt!(1, 0, 1))]
    #[case(vc!(1, 2, 0), pt!(1, 0, 0), pt!(2, 2, 0))]
    #[case(vc!(1, 2, 3), pt!(1, 0, 0), pt!(2, 2, 3))]
    #[case(vc!(1, 4, 2), pt!(5, 1, 7), pt!(6, 5, 9))]
    fn translate_point(
        #[case] displacement: Vector<3>,
        #[case] p: Point<3>,
        #[case] expected: Point<3>,
    ) {
        let matrix = Matrix::translate(&displacement);
        let actual = &matrix * &p;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, vc!(1, 1, 1), vc!(0, 0, 0))]
    #[case(1.0, 1.0, 1.0, vc!(1, 1, 1), vc!(1, 1, 1))]
    #[case(2.0, 1.0, 1.0, vc!(1, 1, 1), vc!(2, 1, 1))]
    #[case(1.0, 2.0, 1.0, vc!(1, 1, 1), vc!(1, 2, 1))]
    #[case(1.0, 1.0, 2.0, vc!(1, 1, 1), vc!(1, 1, 2))]
    #[case(2.0, 3.0, 2.0, vc!(2, 3, 4), vc!(4, 9, 8))]
    fn scale_vector(
        #[case] sx: f64,
        #[case] sy: f64,
        #[case] sz: f64,
        #[case] v: Vector<3>,
        #[case] expected: Vector<3>,
    ) {
        let matrix = Matrix::scale(sx, sy, sz);
        let actual = &matrix * &v;

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(pt!(1, 0, 0), 0.0, pt!(1, 0, 0))]
    #[case(pt!(0, 1, 0), 0.0, pt!(0, 1, 0))]
    #[case(pt!(0, 0, 1), 0.0, pt!(0, 0, 1))]
    #[case(pt!(1, 0, 0), 90.0, pt!(1, 0, 0))]
    #[case(pt!(0, 1, 0), 90.0, pt!(0, 0, 1))]
    #[case(pt!(0, 0, 1), 90.0, pt!(0, -1, 0))]
    #[case(pt!(0, -1, 0), 90.0, pt!(0, 0, -1))]
    #[case(pt!(0, 0, -1), 90.0, pt!(0, 1, 0))]
    #[case(pt!(0, 0, -1), 180.0, pt!(0, 0, 1))]
    #[case(pt!(0, 0, 1), 180.0, pt!(0, 0, -1))]
    fn rotate_point_around_x(
        #[case] p: Point<3>,
        #[case] degrees: f64,
        #[case] expected: Point<3>,
        #[values(1.0, 2.0, 5.0, -1.0)] factor: f64,
    ) {
        let scaled_p = pt!(p.x() * factor, p.y() * factor, p.z() * factor);
        let scaled_expected = pt!(
            expected.x() * factor,
            expected.y() * factor,
            expected.z() * factor
        );
        let matrix = Matrix::rotate_around_x(Angle::degrees(degrees));
        let actual = &matrix * &scaled_p;

        assert_eq!(approx(scaled_expected), actual);
    }

    #[rstest]
    #[case(pt!(1, 0, 0), 0.0, pt!(1, 0, 0))]
    #[case(pt!(0, 1, 0), 0.0, pt!(0, 1, 0))]
    #[case(pt!(0, 0, 1), 0.0, pt!(0, 0, 1))]
    #[case(pt!(1, 0, 0), 90.0, pt!(0, 0, -1))]
    #[case(pt!(0, 1, 0), 90.0, pt!(0, 1, 0))]
    #[case(pt!(0, 0, 1), 90.0, pt!(1, 0, 0))]
    #[case(pt!(0, 0, -1), 90.0, pt!(-1, 0, 0))]
    #[case(pt!(-1, 0, 0), 90.0, pt!(0, 0, 1))]
    fn rotate_point_around_y(
        #[case] p: Point<3>,
        #[case] degrees: f64,
        #[case] expected: Point<3>,
        #[values(1.0, 2.0, 5.0, -1.0)] factor: f64,
    ) {
        let scaled_p = pt!(p.x() * factor, p.y() * factor, p.z() * factor);
        let scaled_expected = pt!(
            expected.x() * factor,
            expected.y() * factor,
            expected.z() * factor
        );
        let matrix = Matrix::rotate_around_y(Angle::degrees(degrees));
        let actual = &matrix * &scaled_p;

        assert_eq!(approx(scaled_expected), actual);
    }

    #[rstest]
    #[case(pt!(1, 0, 0), 0.0, pt!(1, 0, 0))]
    #[case(pt!(0, 1, 0), 0.0, pt!(0, 1, 0))]
    #[case(pt!(0, 0, 1), 0.0, pt!(0, 0, 1))]
    #[case(pt!(1, 0, 0), 90.0, pt!(0, 1, 0))]
    #[case(pt!(0, 1, 0), 90.0, pt!(-1, 0, 0))]
    #[case(pt!(0, 0, 1), 90.0, pt!(0, 0, 1))]
    #[case(pt!(-1, 0, 0), 90.0, pt!(0, -1, 0))]
    #[case(pt!(0, -1, 0), 90.0, pt!(1, 0, 0))]
    fn rotate_point_around_z(
        #[case] p: Point<3>,
        #[case] degrees: f64,
        #[case] expected: Point<3>,
        #[values(1.0, 2.0, 5.0, -1.0)] factor: f64,
    ) {
        let scaled_p = pt!(p.x() * factor, p.y() * factor, p.z() * factor);
        let scaled_expected = pt!(
            expected.x() * factor,
            expected.y() * factor,
            expected.z() * factor
        );
        let matrix = Matrix::rotate_around_z(Angle::degrees(degrees));
        let actual = &matrix * &scaled_p;

        assert_eq!(approx(scaled_expected), actual);
    }
}
