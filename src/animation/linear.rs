use super::Animation;

pub trait Interpolate {
    fn interpolate(start: &Self, end: &Self, t: f64) -> Self;
}

pub struct Linear<T: Interpolate> {
    pub duration: f64,
    pub start: T,
    pub end: T,
}

impl<T: Interpolate> Animation<T> for Linear<T> {
    fn duration(&self) -> f64 {
        self.duration
    }

    fn at(&self, t: f64) -> T {
        T::interpolate(&self.start, &self.end, t / self.duration)
    }
}

impl Interpolate for f64 {
    fn interpolate(start: &Self, end: &Self, t: f64) -> Self {
        start + (end - start) * t
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    #[case(0.0, 1.0, 1.0, 0.0, 0.0)]
    #[case(0.0, 1.0, 1.0, 0.5, 0.5)]
    #[case(0.0, 1.0, 1.0, 1.0, 1.0)]
    #[case(0.0, 2.0, 1.0, 0.5, 1.0)]
    #[case(0.0, 2.0, 2.0, 1.0, 1.0)]
    #[case(0.0, 2.0, 2.0, 2.0, 2.0)]
    #[case(1.0, 2.0, 2.0, 1.0, 1.5)]
    fn f64_linear(#[case] start: f64, #[case] end: f64, #[case] duration: f64, #[case] t: f64, #[case] expected: f64) {
        let animation = Linear {
            start,
            end,
            duration
        };
        let actual = animation.at(t);

        assert_eq!(expected, actual);
    }
}