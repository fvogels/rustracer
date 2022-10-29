use super::{matrix4d::Matrix4D, vector3d::Vector3D};

pub struct Transformation3D {
    pub matrix: Matrix4D,
    pub inverse_matrix: Matrix4D
}

impl Transformation3D {
    pub fn translate(displacement: &Vector3D) -> Transformation3D {
        let matrix = Matrix4D::translate(displacement);
        let inverse_matrix = Matrix4D::translate(&-displacement);

        Transformation3D { matrix, inverse_matrix }
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Transformation3D {
        let matrix = Matrix4D::scale(sx, sy, sz);
        let inverse_matrix = Matrix4D::scale(sx.recip(), sy.recip(), sz.recip());

        Transformation3D { matrix, inverse_matrix }
    }
}