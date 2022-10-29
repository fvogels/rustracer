use super::{metric::Metric, interval::Linear};

#[derive(Debug, Clone, Copy)]
pub struct Angle {
    size_in_radians: f64,
}

impl Angle {
    pub fn degrees(size_in_degrees: f64) -> Angle {
        let size_in_radians = Angle::degrees_to_radians(size_in_degrees);

        Angle { size_in_radians }
    }

    pub fn radians(size_in_radians: f64) -> Angle {
        Angle { size_in_radians }
    }

    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees / 180.0 * std::f64::consts::PI
    }

    fn radians_to_degrees(radians: f64) -> f64 {
        radians / std::f64::consts::PI * 180.0
    }

    pub fn in_degrees(&self) -> f64 {
        Angle::radians_to_degrees(self.size_in_radians)
    }

    pub fn in_radians(&self) -> f64 {
        self.size_in_radians
    }

    pub fn sin(&self) -> f64 {
        self.in_radians().sin()
    }

    pub fn cos(&self) -> f64 {
        self.in_radians().cos()
    }
}

impl std::ops::Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Angle::radians(self.in_radians() + rhs.in_radians())
    }
}

impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.size_in_radians += rhs.in_radians();
    }
}

impl std::ops::Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        Angle::radians(-self.in_radians())
    }
}

impl std::ops::Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Angle::radians(self.in_radians() - rhs.in_radians())
    }
}

impl std::ops::SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.size_in_radians -= rhs.in_radians();
    }
}

impl std::ops::Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Self::Output {
        Angle::radians(self.in_radians() * rhs)
    }
}

impl std::ops::MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        self.size_in_radians *= rhs;
    }
}

impl std::cmp::PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.in_radians() == other.in_radians()
    }
}

impl std::cmp::PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size_in_radians.partial_cmp(&other.size_in_radians)
    }
}

impl Metric for Angle {
    fn distance(&self, rhs: &Self) -> f64 {
        self.in_radians().distance(&rhs.in_radians())
    }
}

impl Linear for Angle {
    fn position(lower: &Self, upper: &Self, x: &Self) -> f64 {
        f64::position(&lower.in_radians(), &upper.in_radians(), &x.in_radians())
    }

    fn from_position(lower: &Self, upper: &Self, t: f64) -> Self {
        let radians = f64::from_position(&lower.in_radians(), &upper.in_radians(), t);
        Angle::radians(radians)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::math::{approx::approx, interval::{Interval, IntervalMapper}};

    #[rstest]
    #[case(Interval::new(0.0, 1.0), Interval::new(Angle::degrees(-180.0), Angle::degrees(180.0)), 0.0, Angle::degrees(-180.0))]
    #[case(Interval::new(0.0, 1.0), Interval::new(Angle::degrees(-180.0), Angle::degrees(180.0)), 0.5, Angle::degrees(0.0))]
    #[case(Interval::new(0.0, 1.0), Interval::new(Angle::degrees(-180.0), Angle::degrees(180.0)), 1.0, Angle::degrees(180.0))]
    #[case(Interval::new(-1.0, 1.0), Interval::new(Angle::degrees(-180.0), Angle::degrees(180.0)), 0.0, Angle::degrees(0.0))]
    #[case(Interval::new(-1.0, 1.0), Interval::new(Angle::degrees(0.0), Angle::degrees(90.0)), 0.0, Angle::degrees(45.0))]
    #[case(Interval::new(-1.0, 1.0), Interval::new(Angle::degrees(0.0), Angle::degrees(90.0)), -1.0, Angle::degrees(0.0))]
    #[case(Interval::new(-1.0, 1.0), Interval::new(Angle::degrees(0.0), Angle::degrees(90.0)), 1.0, Angle::degrees(90.0))]
    fn interval_mapping_f64_to_angle(#[case] source: Interval<f64>, #[case] target: Interval<Angle>, #[case] value: f64, #[case] angle: Angle) {
        let mapper = IntervalMapper::new(source, target);

        assert_eq!(approx(angle), mapper.map(value));
        assert_eq!(approx(value), mapper.inverse_map(angle));
    }
}
