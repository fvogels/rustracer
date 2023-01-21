use super::sampler::Sampler2D;
use crate::{math::{Point, Position, Rasterizer, Rectangle}, util::Refine};

pub struct StratifiedSampler2D { }

struct SampleRefiner {
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
    fn sample(&self, rectangle: Rectangle<2>) -> Box<dyn Refine<Point<2>>> {
        let rasterizer = Rasterizer::<2>::new(rectangle, 1, 1);

        Box::new(SampleRefiner {
            rasterizer,
            x: 0,
            y: 0,
        })
    }
}

impl Refine<Point<2>> for SampleRefiner {
    fn current(&self) -> Point<2> {
        self.rasterizer.at(Position::<2>::cartesian(self.x, self.y)).center()
    }

    fn refine(&mut self) {
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

        assert_eq!(approx(pt!(8, 8)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(4, 4)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(12, 4)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(4, 12)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(12, 12)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(2, 2)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(6, 2)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(10, 2)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(14, 2)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(2, 6)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(6, 6)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(10, 6)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(14, 6)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(2, 10)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(6, 10)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(10, 10)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(14, 10)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(2, 14)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(6, 14)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(10, 14)), iterator.current());
        iterator.refine();
        assert_eq!(approx(pt!(14, 14)), iterator.current());
        iterator.refine();
    }
}
