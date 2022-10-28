pub trait Metric {
    fn distance(&self, rhs: &Self) -> f64;
}

impl Metric for f64 {
    fn distance(&self, rhs: &Self) -> f64 {
        (rhs - self).abs()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0, 0.0)]
    #[case(0.0, 1.0, 1.0)]
    #[case(2.0, 1.0, 1.0)]
    #[case(0.2, 0.3, 0.3 - 0.2)]
    fn metric_f64(#[case] x: f64, #[case] y: f64, #[case] expected: f64) {
        assert_eq!(expected, x.distance(&y));
        assert_eq!(expected, y.distance(&x));
    }
}
