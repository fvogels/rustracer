use std::ops::Mul;

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