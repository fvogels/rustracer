use super::Point;
use super::Vector;

pub struct Ray {
    pub origin: Point<3>,
    pub direction: Vector<3>,
}

impl Ray {
    pub fn new(origin: Point<3>, direction: Vector<3>) -> Self {
        Ray { origin, direction }
    }

    pub fn through(origin: Point<3>, point: Point<3>) -> Self {
        let direction = point - origin;

        Ray::new(origin, direction)
    }

    pub fn at(&self, t: f64) -> Point<3> {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::Ray;
    use crate::math::{Point, pt};
    use crate::math::{Vector, vc};
    use rstest::rstest;

    #[rstest]
    #[case(pt!(0, 0, 0), vc!(1, 0, 0), 1.0, pt!(1, 0, 0))]
    #[case(pt!(0, 0, 0), vc!(1, 0, 0), 2.0, pt!(2, 0, 0))]
    #[case(pt!(1, 0, 0), vc!(1, 0, 0), 1.0, pt!(2, 0, 0))]
    #[case(pt!(1, 0, 0), vc!(2, 0, 0), 1.0, pt!(3, 0, 0))]
    #[case(pt!(1, 2, 3), vc!(1, 1, 1), 1.0, pt!(2, 3, 4))]
    #[case(pt!(1, 2, 3), vc!(1, 1, 1), 2.0, pt!(3, 4, 5))]
    fn at(
        #[case] position: Point<3>,
        #[case] direction: Vector<3>,
        #[case] t: f64,
        #[case] expected: Point<3>,
    ) {
        let actual = Ray::new(position, direction).at(t);

        assert_eq!(expected, actual);
    }
}
