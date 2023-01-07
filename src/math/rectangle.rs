use super::point::{pt, Point};
use super::vector::Vector;

pub struct Rectangle<const N: usize> {
    pub origin: Point<N>,
    pub x_axis: Vector<N>,
    pub y_axis: Vector<N>,
}

impl<const N: usize> Rectangle<N> {
    pub fn new(origin: Point<N>, x_axis: Vector<N>, y_axis: Vector<N>) -> Self {
        Rectangle {
            origin,
            x_axis,
            y_axis,
        }
    }

    pub fn from_relative(&self, p: &Point<2>) -> Point<N> {
        self.origin + self.x_axis * p.x() + self.y_axis * p.y()
    }

    pub fn center(&self) -> Point<N> {
        self.from_relative(&pt!(0.5, 0.5))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use crate::math::{pt, vc};
    use rstest::rstest;

    #[rstest]
    #[case(Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1)), pt!(0, 0), pt!(0, 0))]
    #[case(Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1)), pt!(1, 0), pt!(1, 0))]
    #[case(Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1)), pt!(0, 1), pt!(0, 1))]
    #[case(Rectangle::new(pt!(0, 0), vc!(1, 0), vc!(0, 1)), pt!(1, 1), pt!(1, 1))]
    #[case(Rectangle::new(pt!(0, 0), vc!(2, 0), vc!(0, 4)), pt!(1, 0), pt!(2, 0))]
    #[case(Rectangle::new(pt!(0, 0), vc!(2, 0), vc!(0, 4)), pt!(0, 1), pt!(0, 4))]
    fn from_relative(
        #[case] rectangle: Rectangle<2>,
        #[case] p: Point<2>,
        #[case] expected: Point<2>,
    ) {
        let actual = rectangle.from_relative(&p);

        assert_eq!(expected, actual);
    }
}
