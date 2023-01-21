use std::{ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign}, net::AddrParseError};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct TimeStamp {
    pub(super) value: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Duration {
    pub(super) value: f64,
}

impl TimeStamp {
    pub fn new(value: f64) -> Self {
        TimeStamp { value }
    }

    pub fn zero() -> Self {
        Self::new(0.0)
    }
}

impl Duration {
    pub fn from_seconds(value: f64) -> Self {
        Duration { value }
    }

    pub fn in_seconds(&self) -> f64 {
        self.value
    }
}

impl Add<Duration> for TimeStamp {
    type Output = TimeStamp;

    fn add(self, rhs: Duration) -> Self::Output {
        TimeStamp { value: self.value + rhs.value }
    }
}

impl AddAssign<Duration> for TimeStamp {
    fn add_assign(&mut self, rhs: Duration) {
        self.value += rhs.value;
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration { value: self.value + rhs.value }
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.value += rhs.value;
    }
}

impl Sub for TimeStamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration { value: self.value - rhs.value }
    }
}

impl Mul<f64> for Duration {
    type Output = Duration;

    fn mul(self, rhs: f64) -> Self::Output {
        Duration { value: self.value * rhs }
    }
}

impl MulAssign<f64> for Duration {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl Div<f64> for Duration {
    type Output = Duration;

    fn div(self, rhs: f64) -> Self::Output {
        Duration { value: self.value / rhs }
    }
}

impl DivAssign<f64> for Duration {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}