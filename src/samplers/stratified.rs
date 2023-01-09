use super::sampler::Sampler2D;
use crate::math::{Point, Position, Rasterizer, Rectangle};

pub struct StratifiedSampler2D { }

struct SampleIterator {
    rasterizer: Rasterizer<2>,
    x: i32,
    y: i32,
}

impl StratifiedSampler2D {
    pub fn new() -> Self {
        StratifiedSampler2D { }
    }
}

impl Sampler2D for StratifiedSampler2D {
    fn sample(&self, rectangle: Rectangle<2>) -> Box<dyn Iterator<Item = Point<2>>> {
        let rasterizer = Rasterizer::<2>::new(rectangle, 1, 1);

        Box::new(SampleIterator {
            rasterizer,
            x: 0,
            y: 0,
        })
    }
}

impl Iterator for SampleIterator {
    type Item = Point<2>;

    fn next(&mut self) -> Option<Self::Item> {
        let rectangle = self.rasterizer.at(Position::<2>::cartesian(self.x, self.y));

        self.x += 1;
        if self.x == self.rasterizer.width as i32 {
            self.x = 0;
            self.y += 1;

            if self.y == self.rasterizer.height as i32 {
                self.y = 0;
                self.rasterizer.width *= 2;
                self.rasterizer.height *= 2;
            }
        }

        Some(rectangle.center())
    }
}

mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    use crate::math::approx;
    #[cfg(test)]
    use crate::{
        math::{pt, vc},
    };

    #[cfg(test)]
    fn rectangle(origin: Point<2>, width: u32, height: u32) -> Rectangle<2> {
        let x_axis = vc!(width, 0);
        let y_axis = vc!(0, height);

        Rectangle {
            origin,
            x_axis,
            y_axis,
        }
    }

    #[rstest]
    fn sampling1() {
        let sampler = StratifiedSampler2D::new();
        let rectangle = rectangle(pt!(0, 0), 16, 16);
        let mut iterator = sampler.sample(rectangle);

        assert_eq!(approx(pt!(8, 8)), iterator.next().unwrap());
        assert_eq!(approx(pt!(4, 4)), iterator.next().unwrap());
        assert_eq!(approx(pt!(12, 4)), iterator.next().unwrap());
        assert_eq!(approx(pt!(4, 12)), iterator.next().unwrap());
        assert_eq!(approx(pt!(12, 12)), iterator.next().unwrap());
        assert_eq!(approx(pt!(2, 2)), iterator.next().unwrap());
        assert_eq!(approx(pt!(6, 2)), iterator.next().unwrap());
        assert_eq!(approx(pt!(10, 2)), iterator.next().unwrap());
        assert_eq!(approx(pt!(14, 2)), iterator.next().unwrap());
        assert_eq!(approx(pt!(2, 6)), iterator.next().unwrap());
        assert_eq!(approx(pt!(6, 6)), iterator.next().unwrap());
        assert_eq!(approx(pt!(10, 6)), iterator.next().unwrap());
        assert_eq!(approx(pt!(14, 6)), iterator.next().unwrap());
        assert_eq!(approx(pt!(2, 10)), iterator.next().unwrap());
        assert_eq!(approx(pt!(6, 10)), iterator.next().unwrap());
        assert_eq!(approx(pt!(10, 10)), iterator.next().unwrap());
        assert_eq!(approx(pt!(14, 10)), iterator.next().unwrap());
        assert_eq!(approx(pt!(2, 14)), iterator.next().unwrap());
        assert_eq!(approx(pt!(6, 14)), iterator.next().unwrap());
        assert_eq!(approx(pt!(10, 14)), iterator.next().unwrap());
        assert_eq!(approx(pt!(14, 14)), iterator.next().unwrap());
    }
}
