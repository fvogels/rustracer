use super::position2d::Position2D;
use super::rectangle2d::Rectangle2D;

pub struct Rasterizer2D {
    rectangle: Rectangle2D,
    width: u32,
    height: u32,
}

impl Rasterizer2D {
    pub fn new(rectangle: Rectangle2D, width: u32, height: u32) -> Rasterizer2D {
        Rasterizer2D {
            rectangle,
            width,
            height,
        }
    }

    pub fn at(&self, position: Position2D) -> Rectangle2D {
        assert!(0 <= position.x && position.x < self.width as i32);
        assert!(0 <= position.y && position.y < self.height as i32);

        let x_axis = self.rectangle.x_axis * (1.0 / (self.width as f64));
        let y_axis = self.rectangle.y_axis * (1.0 / (self.width as f64));
        let origin =
            self.rectangle.origin + x_axis * (position.x as f64) + y_axis * (position.y as f64);

        Rectangle2D {
            origin,
            x_axis,
            y_axis,
        }
    }
}
