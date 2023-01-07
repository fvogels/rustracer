use crate::math::{Point, Rectangle};

pub trait Sampler2D<'a> {
    fn sample(&self, rectangle: &'a Rectangle<2>) -> Box<dyn Iterator<Item = Point<2>> + 'a>;
}
