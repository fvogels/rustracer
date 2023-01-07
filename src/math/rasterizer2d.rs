use super::Position;
use super::Rectangle;

pub struct Rasterizer2D<'a> {
    rectangle: &'a Rectangle<2>,
    width: u32,
    height: u32,
}

impl<'a> Rasterizer2D<'a> {
    pub fn new(rectangle: &'a Rectangle<2>, width: u32, height: u32) -> Self {
        Rasterizer2D {
            rectangle,
            width,
            height,
        }
    }

    pub fn at(&self, position: Position<2>) -> Rectangle<2> {
        debug_assert!(0 <= position.x() && position.x() < self.width as i32);
        debug_assert!(0 <= position.y() && position.y() < self.height as i32);

        let x_axis = self.rectangle.x_axis * (self.width as f64).recip();
        let y_axis = self.rectangle.y_axis * (self.height as f64).recip();
        let origin =
            self.rectangle.origin + x_axis * (position.x() as f64) + y_axis * (position.y() as f64);

        Rectangle::new(origin, x_axis, y_axis)
    }
}
