use super::sampler::Sampler2D;
use crate::math::{Point, Rectangle};

pub struct SingleSampler2D {}

struct SampleIterator {
    data: Option<Point<2>>,
}

impl SingleSampler2D {
    pub fn new() -> Self {
        SingleSampler2D {}
    }
}

impl<'a> Sampler2D<'a> for SingleSampler2D {
    fn sample(&self, rectangle: &Rectangle<2>) -> Box<dyn Iterator<Item = Point<2>>> {
        let data = Some(rectangle.center());

        Box::new(SampleIterator { data })
    }
}

impl Iterator for SampleIterator {
    type Item = Point<2>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.take()
    }
}
