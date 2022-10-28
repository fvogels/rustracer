use super::rectangle2d::Rectangle2D;

pub struct Rasterizer2D {
    rectangle: Rectangle2D,
    width: u32,
    height: u32,
}

impl Rasterizer2D {
    pub fn new(rectangle: Rectangle2D, width: u32, height: u32) -> Rasterizer2D {
        Rasterizer2D { rectangle, width, height }
    }
}
