use super::{Vector, Point};

pub struct CoordinateSystem3D {
    pub origin: Point<3>,
    pub x_axis: Vector<3>,
    pub y_axis: Vector<3>,
    pub z_axis: Vector<3>,
}
