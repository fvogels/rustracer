use super::{Vector, Point, Matrix};

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
}
