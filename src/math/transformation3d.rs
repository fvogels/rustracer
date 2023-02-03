use super::{angle::Angle, Matrix, Vector};

#[derive(Copy, Clone)]
pub struct Transformation3D {
    pub matrix: Matrix<4, 4>,
    pub inverse_matrix: Matrix<4, 4>,
}

impl Transformation3D {
    pub fn translate(displacement: &Vector<3>) -> Transformation3D {
        let matrix = Matrix::translate(displacement);
        let inverse_matrix = Matrix::translate(&-displacement);

        Transformation3D {
            matrix,
            inverse_matrix,
        }
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Transformation3D {
        let matrix = Matrix::scale(sx, sy, sz);
        let inverse_matrix = Matrix::scale(sx.recip(), sy.recip(), sz.recip());

        Transformation3D {
            matrix,
            inverse_matrix,
        }
    }

    pub fn rotate_around_x(angle: Angle) -> Transformation3D {
        let matrix = Matrix::rotate_around_x(angle);
        let inverse_matrix = Matrix::rotate_around_x(-angle);

        Transformation3D {
            matrix,
            inverse_matrix,
        }
    }

    pub fn rotate_around_y(angle: Angle) -> Transformation3D {
        let matrix = Matrix::rotate_around_y(angle);
        let inverse_matrix = Matrix::rotate_around_y(-angle);

        Transformation3D {
            matrix,
            inverse_matrix,
        }
    }

    pub fn rotate_around_z(angle: Angle) -> Transformation3D {
        let matrix = Matrix::rotate_around_z(angle);
        let inverse_matrix = Matrix::rotate_around_z(-angle);

        Transformation3D {
            matrix,
            inverse_matrix,
        }
    }

    pub fn compose(&self, other: &Transformation3D) -> Transformation3D {
        let matrix = &self.matrix * &other.matrix;
        let inverse_matrix = &other.inverse_matrix * &self.inverse_matrix;

        Transformation3D { matrix, inverse_matrix }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::math::{approx::approx, vc};

    #[rstest]
    #[case(vc!(0, 0, 0))]
    #[case(vc!(1, 0, 0))]
    #[case(vc!(0, 1, 0))]
    #[case(vc!(0, 0, 1))]
    #[case(vc!(5, 4, 3))]
    fn translate_inverse_matrix(#[case] displacement: Vector<3>) {
        let transformation = Transformation3D::translate(&displacement);

        assert_eq!(
            approx(Matrix::identity()),
            &transformation.matrix * &transformation.inverse_matrix
        );
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0)]
    #[case(2.0, 1.0, 1.0)]
    #[case(1.0, 2.0, 1.0)]
    #[case(1.0, 1.0, 2.0)]
    #[case(3.0, 4.0, 5.0)]
    fn scale_inverse_matrix(#[case] sx: f64, #[case] sy: f64, #[case] sz: f64) {
        let transformation = Transformation3D::scale(sx, sy, sz);

        assert_eq!(
            approx(Matrix::identity()),
            &transformation.matrix * &transformation.inverse_matrix
        );
    }

    #[rstest]
    fn rotate_x_inverse_matrix(#[values(0.0, 90.0, 180.0, 45.0, 60.0, 135.0)] degrees: f64) {
        let transformation = Transformation3D::rotate_around_x(Angle::degrees(degrees));

        assert_eq!(
            approx(Matrix::identity()),
            &transformation.matrix * &transformation.inverse_matrix
        );
    }

    #[rstest]
    fn rotate_y_inverse_matrix(#[values(0.0, 90.0, 180.0, 45.0, 60.0, 135.0)] degrees: f64) {
        let transformation = Transformation3D::rotate_around_y(Angle::degrees(degrees));

        assert_eq!(
            approx(Matrix::identity()),
            &transformation.matrix * &transformation.inverse_matrix
        );
    }

    #[rstest]
    fn rotate_z_inverse_matrix(#[values(0.0, 90.0, 180.0, 45.0, 60.0, 135.0)] degrees: f64) {
        let transformation = Transformation3D::rotate_around_z(Angle::degrees(degrees));

        assert_eq!(
            approx(Matrix::identity()),
            &transformation.matrix * &transformation.inverse_matrix
        );
    }
}
