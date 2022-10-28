use super::metric::Metric;

pub trait Approx {
    fn approx(&self, rhs: &Self) -> bool {
        self.approx_eps(rhs, 0.00001)
    }

    fn approx_eps(&self, rhs: &Self, epsilon: f64) -> bool;
}

impl<T : Metric> Approx for T {
    fn approx_eps(&self, rhs: &Self, epsilon: f64) -> bool {
        rhs.distance(rhs) < epsilon
    }
}
