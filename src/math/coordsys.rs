use super::{Matrix, Point, Vector, transformation3d::Transformation3D};

pub struct CoordinateSystem3D {
    pub origin: Point<3>,
    pub x_axis: Vector<3>,
    pub y_axis: Vector<3>,
    pub z_axis: Vector<3>,
}

impl CoordinateSystem3D {
    pub fn transform(&mut self, matrix: &Matrix<4, 4>) {
        self.origin = matrix * &self.origin;
        self.x_axis = matrix * &self.x_axis;
        self.y_axis = matrix * &self.y_axis;
        self.z_axis = matrix * &self.z_axis;
    }

    pub fn as_transformation(&self) -> Transformation3D {
        let matrix = Matrix::<4, 4>::from_coordinate_system(&self.origin, &self.x_axis, &self.y_axis, &self.z_axis);
        let inverse_matrix = matrix.transpose();

        Transformation3D { matrix, inverse_matrix }
    }
}
