use crate::math::{rectangle2d::Rectangle2D, point2d::Point2D};

pub trait Sampler2D<'a> {
    fn sample(&self, rectangle: &'a Rectangle2D) -> Box<dyn Iterator<Item=Point2D> + 'a>;
}
