use crate::math::{Point, Rectangle};

pub trait Sampler2D {
    fn sample(&self, rectangle: Rectangle<2>) -> Box<dyn Iterator<Item = Point<2>>>;
}
