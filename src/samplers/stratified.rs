use crate::math::{rectangle2d::Rectangle2D, point2d::Point2D, rasterizer2d::Rasterizer2D, position2d::Position2D};
use super::sampler::Sampler2D;

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
    pub fn new(horizontal: u32, vertical: u32) -> StratifiedSampler2D {
        StratifiedSampler2D { horizontal, vertical }
    }
}

impl<'a> Sampler2D<'a> for StratifiedSampler2D {
    fn sample(&self, rectangle: &'a Rectangle2D) -> Box<dyn Iterator<Item=Point2D> + 'a> {
        let rasterizer = Rasterizer2D::new(rectangle, self.horizontal, self.vertical);
        let row = 0;
        let col = 0;
        let nrows = self.vertical;
        let ncols = self.horizontal;

        Box::new(SampleIterator { rasterizer, row, col, nrows, ncols })
    }
}

impl<'a> Iterator for SampleIterator<'a> {
    type Item = Point2D;

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
    use crate::{math::{vector2d::v2, point2d::p2}, util::algorithms::assert_same_elements};

    #[cfg(test)]
    fn rectangle(origin: Point2D, width: u32, height: u32) -> Rectangle2D {
        let x_axis = v2!(width, 0);
        let y_axis = v2!(0, height);

        Rectangle2D { origin, x_axis, y_axis }
    }

    #[rstest]
    fn sampling1() {
        let sampler = StratifiedSampler2D::new(1, 1);
        let rectangle = rectangle(p2!(0, 0), 2, 2);
        let expected = vec![p2!(1, 1)];
        let actual: Vec<Point2D> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling2() {
        let sampler = StratifiedSampler2D::new(1, 1);
        let rectangle = rectangle(p2!(0, 0), 4, 6);
        let expected = vec![p2!(2, 3)];
        let actual: Vec<Point2D> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling3() {
        let sampler = StratifiedSampler2D::new(2, 1);
        let rectangle = rectangle(p2!(0, 0), 4, 2);
        let expected = vec![p2!(1, 1), p2!(3, 1)];
        let actual: Vec<Point2D> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling4() {
        let sampler = StratifiedSampler2D::new(2, 1);
        let rectangle = rectangle(p2!(1, 0), 4, 2);
        let expected = vec![p2!(2, 1), p2!(4, 1)];
        let actual: Vec<Point2D> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }

    #[rstest]
    fn sampling5() {
        let sampler = StratifiedSampler2D::new(2, 3);
        let rectangle = rectangle(p2!(1, 0), 4, 6);
        let expected = vec![p2!(2, 1), p2!(4, 1), p2!(2, 3), p2!(4, 3), p2!(2, 5), p2!(4, 5)];
        let actual: Vec<Point2D> = sampler.sample(&rectangle).collect();

        assert_same_elements!(expected, actual);
    }
}