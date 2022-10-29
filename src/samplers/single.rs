use super::sampler::Sampler2D;
use crate::math::{point2d::Point2D, rectangle2d::Rectangle2D};

pub struct SingleSampler2D {}

struct SampleIterator {
    data: Option<Point2D>,
}

impl SingleSampler2D {
    pub fn new() -> SingleSampler2D {
        SingleSampler2D {}
    }
}

impl<'a> Sampler2D<'a> for SingleSampler2D {
    fn sample(&self, rectangle: &Rectangle2D) -> Box<dyn Iterator<Item = Point2D>> {
        let data = Some(rectangle.center());

        Box::new(SampleIterator { data })
    }
}

impl Iterator for SampleIterator {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.take()
    }
}
