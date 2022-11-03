use std::ops::Sub;

pub trait Linear {
    fn position(lower: &Self, upper: &Self, x: &Self) -> f64;

    fn from_position(lower: &Self, upper: &Self, t: f64) -> Self;
}

pub struct Interval<T> {
    pub lower_bound: T,
    pub upper_bound: T,
}

pub struct IntervalMapper<T: Linear, U: Linear> {
    pub source: Interval<T>,
    pub target: Interval<U>,
}

impl<T> Interval<T> {
    pub fn new(lower_bound: T, upper_bound: T) -> Interval<T> {
        Interval {
            lower_bound,
            upper_bound,
        }
    }
}

impl<T: Linear> Interval<T> {
    pub fn contains(&self, x: T) -> bool {
        let t = self.position(x);
        0.0 <= t && t <= 1.0
    }

    pub fn position(&self, x: T) -> f64 {
        T::position(&self.lower_bound, &self.upper_bound, &x)
    }

    pub fn from_position(&self, t: f64) -> T {
        T::from_position(&self.lower_bound, &self.upper_bound, t)
    }
}

impl<'a, T: 'a> Interval<T>
where
    &'a T: Sub<&'a T, Output = T>,
{
    pub fn width(&'a self) -> T {
        &self.upper_bound - &self.lower_bound
    }
}

impl<T: Linear, U: Linear> IntervalMapper<T, U> {
    pub fn new(source: Interval<T>, target: Interval<U>) -> IntervalMapper<T, U> {
        IntervalMapper { source, target }
    }

    pub fn map(&self, x: T) -> U {
        let t = self.source.position(x);
        self.target.from_position(t)
    }

    pub fn inverse_map(&self, y: U) -> T {
        let t = self.target.position(y);
        self.source.from_position(t)
    }
}

impl Linear for f64 {
    fn position(lower: &Self, upper: &Self, x: &Self) -> f64 {
        (x - lower) / (upper - lower)
    }

    fn from_position(lower: &Self, upper: &Self, t: f64) -> Self {
        lower + (upper - lower) * t
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::math::approx::approx;

    #[rstest]
    #[case(0.0, 1.0, 1.0)]
    #[case(0.0, 2.0, 2.0)]
    #[case(1.0, 2.0, 1.0)]
    #[case(1.0, 11.0, 10.0)]
    fn width_f64(#[case] lower: f64, #[case] upper: f64, #[case] expected: f64) {
        let interval = Interval::new(lower, upper);

        assert_eq!(approx(expected), interval.width());
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(1.0, 2.0)]
    #[case(0.5, 1.5)]
    fn f64_to_f64_1(#[case] x: f64, #[case] y: f64) {
        let source = Interval::new(0.0, 1.0);
        let target = Interval::new(1.0, 2.0);
        let mapper = IntervalMapper::new(source, target);

        assert_eq!(approx(y), mapper.map(x));
        assert_eq!(approx(x), mapper.inverse_map(y));
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(5.0, 50.0)]
    #[case(10.0, 100.0)]
    fn f64_to_f64_2(#[case] x: f64, #[case] y: f64) {
        let source = Interval::new(0.0, 10.0);
        let target = Interval::new(0.0, 100.0);
        let mapper = IntervalMapper::new(source, target);

        assert_eq!(approx(y), mapper.map(x));
        assert_eq!(approx(x), mapper.inverse_map(y));
    }
}
