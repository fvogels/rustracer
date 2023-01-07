use super::Position;
use super::Rectangle;

pub struct Rasterizer<'a, const N: usize> {
    rectangle: &'a Rectangle<N>,
    width: u32,
    height: u32,
}

impl<'a, const N: usize> Rasterizer<'a, N> {
    pub fn new(rectangle: &'a Rectangle<N>, width: u32, height: u32) -> Self {
        Rasterizer {
            rectangle,
            width,
            height,
        }
    }

    pub fn at(&self, position: Position<2>) -> Rectangle<N> {
        let x_axis = self.rectangle.x_axis * (1.0 / (self.width as f64));
        let y_axis = self.rectangle.y_axis * (1.0 / (self.height as f64));
        let origin =
            self.rectangle.origin + x_axis * (position.x() as f64) + y_axis * (position.y() as f64);

        Rectangle::new(origin, x_axis, y_axis)
    }
}
