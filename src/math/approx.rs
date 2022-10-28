use super::metric::Metric;

pub trait Approx {
    fn approx(&self, rhs: &Self) -> bool {
        self.approx_eps(rhs, 0.00001)
    }

    fn approx_eps(&self, rhs: &Self, epsilon: f64) -> bool;
}

impl<T : Metric> Approx for T {
    fn approx_eps(&self, rhs: &Self, epsilon: f64) -> bool {
        self.distance(rhs) < epsilon
    }
}

pub struct ApproxWrapper<T : Approx> {
    value: T,
}

pub fn approx<T : Approx>(value : T) -> ApproxWrapper<T> {
    ApproxWrapper { value }
}

impl<T : Approx> PartialEq<T> for ApproxWrapper<T> {
    fn eq(&self, other: &T) -> bool {
        self.value.approx(other)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0, true)]
    #[case(0.0, 0.0000001, true)]
    #[case(8.0, 7.9999999, true)]
    #[case(8.0, 8.0000001, true)]
    #[case(0.0, 1.0, false)]
    fn approx_f64(#[case] x: f64, #[case] y: f64, #[case] expected: bool) {
        assert_eq!(expected, approx(x) == y);
        assert_eq!(expected, approx(y) == x);
    }
}
