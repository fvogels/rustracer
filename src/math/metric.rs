pub trait Metric {
    fn distance(&self, rhs: &Self) -> f64;
}

impl Metric for f64 {
    fn distance(&self, rhs: &Self) -> f64 {
        (rhs - self).abs()
    }
}
