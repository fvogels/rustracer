use super::position3d::Position3D;
use super::rectangle3d::Rectangle3D;

pub struct Rasterizer3D {
    rectangle: Rectangle3D,
    width: u32,
    height: u32,
}

impl Rasterizer3D {
    pub fn new(rectangle: Rectangle3D, width: u32, height: u32) -> Rasterizer3D {
        Rasterizer3D {
            rectangle,
            width,
            height,
        }
    }

    pub fn at(&self, position: Position3D) -> Rectangle3D {
        assert!(0 <= position.x && position.x < self.width as i32);
        assert!(0 <= position.y && position.y < self.height as i32);

        let x_axis = self.rectangle.x_axis * (1.0 / (self.width as f64));
        let y_axis = self.rectangle.y_axis * (1.0 / (self.width as f64));
        let origin =
            self.rectangle.origin + x_axis * (position.x as f64) + y_axis * (position.y as f64);

        Rectangle3D {
            origin,
            x_axis,
            y_axis,
        }
    }
}
