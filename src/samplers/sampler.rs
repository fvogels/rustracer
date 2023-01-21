use crate::{math::{Point, Rectangle}, util::Refine};

pub trait Sampler2D {
    fn sample(&self, rectangle: Rectangle<2>) -> Box<dyn Refine<Point<2>>>;
}
