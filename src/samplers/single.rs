use crate::math::{rectangle2d::Rectangle2D, point2d::Point2D};
use super::sampler::Sampler2D;

pub struct SingleSampler2D { }

struct SampleIterator {
    data: Option<Point2D>,
}

impl Sampler2D for SingleSampler2D {
    fn sample(rectangle: &Rectangle2D) -> Box<dyn Iterator<Item=Point2D>> {
        let data = Some(rectangle.center());

        Box::new(SampleIterator { data })
    }
}

impl Iterator for SampleIterator {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        std::mem::replace(&mut self.data, None)
    }
}
