use crate::math::{rectangle2d::Rectangle2D, point2d::Point2D};

pub trait Sampler2D {
    fn sample(rectangle: &Rectangle2D) -> Box<dyn Iterator<Item=Point2D>>;
}
