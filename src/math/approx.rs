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

struct ApproxWrapper<T : Approx> {
    value: T,
}

impl<T : Approx> PartialEq<T> for ApproxWrapper<T> {
    fn eq(&self, other: &T) -> bool {
        self.value.approx(other)
    }
}
