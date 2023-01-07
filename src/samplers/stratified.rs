use super::sampler::Sampler2D;
use crate::math::{
    Point, position2d::Position2D, rasterizer2d::Rasterizer2D, Rectangle,
};

pub struct StratifiedSampler2D {
    horizontal: u32,
    vertical: u32,
}

struct SampleIterator<'a> {
    rasterizer: Rasterizer2D<'a>,
    row: u32,
    col: u32,
    nrows: u32,
    ncols: u32,
}

impl StratifiedSampler2D {
    pub fn new(horizontal: u32, vertical: u32) -> Self {
        StratifiedSampler2D {
            horizontal,
            vertical,
        }
    }
}

impl<'a> Sampler2D<'a> for StratifiedSampler2D {
    fn sample(&self, rectangle: &'a Rectangle<2>) -> Box<dyn Iterator<Item = Point<2>> + 'a> {
        let rasterizer = Rasterizer2D::new(rectangle, self.horizontal, self.vertical);
        let row = 0;
        let col = 0;
        let nrows = self.vertical;
        let ncols = self.horizontal;

        Box::new(SampleIterator {
            rasterizer,
            row,
            col,
            nrows,
            ncols,
        })
    }
}

impl<'a> Iterator for SampleIterator<'a> {
    type Item = Point<2>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.nrows {
            None
        } else {
            let position = Position2D::new(self.col as i32, self.row as i32);
            let sample = self.rasterizer.at(position).center();
            self.col += 1;

            if self.col == self.ncols {
                self.col = 0;
                self.row += 1;
            }

            Some(sample)
        }
    }
}

mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::{
        math::{pt, vc},
        util::algorithms::assert_same_elements,
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
        let sampler = StratifiedSampler2D::new(1, 1);
        let rectangle = rectangle(pt!(0, 0), 2, 2);
        let expected = vec![pt!(1, 1)];
        let actual: Vec<Point<2>> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling2() {
        let sampler = StratifiedSampler2D::new(1, 1);
        let rectangle = rectangle(pt!(0, 0), 4, 6);
        let expected = vec![pt!(2, 3)];
        let actual: Vec<Point<2>> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling3() {
        let sampler = StratifiedSampler2D::new(2, 1);
        let rectangle = rectangle(pt!(0, 0), 4, 2);
        let expected = vec![pt!(1, 1), pt!(3, 1)];
        let actual: Vec<Point<2>> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling4() {
        let sampler = StratifiedSampler2D::new(2, 1);
        let rectangle = rectangle(pt!(1, 0), 4, 2);
        let expected = vec![pt!(2, 1), pt!(4, 1)];
        let actual: Vec<Point<2>> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling5() {
        let sampler = StratifiedSampler2D::new(2, 3);
        let rectangle = rectangle(pt!(1, 0), 4, 6);
        let expected = vec![
            pt!(2, 1),
            pt!(4, 1),
            pt!(2, 3),
            pt!(4, 3),
            pt!(2, 5),
            pt!(4, 5),
        ];
        let actual: Vec<Point<2>> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }
}
