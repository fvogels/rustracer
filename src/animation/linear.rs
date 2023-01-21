use super::{Animation, Duration, TimeStamp};

pub trait Interpolate {
    fn interpolate(start: &Self, end: &Self, t: f64) -> Self;
}

pub struct LinearAnimation<T: Interpolate> {
    pub duration: Duration,
    pub start: T,
    pub end: T,
}

impl<T: Interpolate> Animation<T> for LinearAnimation<T> {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn at(&self, t: TimeStamp) -> T {
        T::interpolate(&self.start, &self.end, t.value / self.duration.value)
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
        let animation = LinearAnimation {
            start,
            end,
            duration: Duration::from_seconds(duration)
        };
        let actual = animation.at(TimeStamp::new(t));

        assert_eq!(expected, actual);
    }
}