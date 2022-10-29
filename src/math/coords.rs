use super::angle::Angle;

pub struct Cartesian2D {
    pub x: f64,
    pub y: f64,
}


pub struct Polar {
    pub radius: f64,
    pub theta: Angle,
}

pub struct Cartesian3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Spherical {
    pub radius: f64,
    pub azimuth: Angle,
    pub elevation: Angle,
}

impl Cartesian2D {
    pub fn to_polar(&self) -> Polar {
        let radius = (self.x.powi(2) + self.y.powi(2)).sqrt();
        let theta = Angle::radians(self.y.atan2(self.x));

        Polar { radius, theta }
    }
}

impl Polar {
    pub fn to_cartesian2d(&self) -> Cartesian2D {
        let x = self.radius * self.theta.cos();
        let y = self.radius * self.theta.sin();

        Cartesian2D { x, y }
    }
}

impl Cartesian3D {
    pub fn to_spherical(&self) -> Spherical {
        let radius = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        let azimuth = Angle::radians(-self.z.atan2(self.x));
        let elevation = if radius > 0.0 {
            Angle::degrees(90.0) - Angle::radians((self.y / radius).acos())
         } else {
            Angle::radians(0.0)
         };

        debug_assert!(radius >= 0.0);
        debug_assert!(Angle::degrees(-180.0) <= azimuth && azimuth <= Angle::degrees(180.0));
        debug_assert!(Angle::degrees(-90.0) <= elevation && elevation <= Angle::degrees(90.0));

        Spherical { radius, azimuth, elevation }
    }
}

impl Spherical {
    pub fn to_cartesian3d(&self) -> Cartesian3D {
        let x = self.radius * self.azimuth.cos() * self.elevation.cos();
        let y = self.radius * self.elevation.sin();
        let z = -self.radius * self.azimuth.sin() * self.elevation.cos();

        Cartesian3D { x, y, z }
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
    fn cartesian2d_polar_back_and_forth(#[values(0.0, 1.0, -1.0, 2.0, -2.0, 5.0, -11.0)] x: f64, #[values(0.0, 1.0, -1.0, 2.0, -2.0, 5.0, -11.0)] y: f64) {
        let cartesian = Cartesian2D { x, y };
        let polar = cartesian.to_polar();
        let cartesian2 = polar.to_cartesian2d();

        assert_eq!(approx(cartesian.x), cartesian2.x);
        assert_eq!(approx(cartesian.y), cartesian2.y);
    }

    #[rstest]
    #[case(Cartesian3D {x: 1.0, y: 0.0, z: 0.0}, Spherical {radius: 1.0, azimuth: Angle::degrees(0.0), elevation: Angle::degrees(0.0)})]
    fn cartesian3d_to_spherical(#[case] cartesian: Cartesian3D, #[case] spherical: Spherical) {
        let actual = cartesian.to_spherical();

        assert_eq!(approx(actual.radius), spherical.radius, "Wrong radius");
        assert_eq!(approx(actual.azimuth), spherical.azimuth, "Wrong azimuth");
        assert_eq!(approx(actual.elevation), spherical.elevation, "Wrong elevation");
    }

    #[rstest]
    fn cartesian3d_spherical_back_and_forth(
        #[values(0.0, 1.0, -1.0, 2.0, -2.0, 5.0, -11.0)] x: f64,
        #[values(0.0, 1.0, -1.0, 2.0, -2.0, 5.0, -11.0)] y: f64,
        #[values(0.0, 1.0, -1.0, 2.0, -2.0, 5.0, -11.0)] z: f64) {
        let cartesian = Cartesian3D { x, y, z };
        let polar = cartesian.to_spherical();
        let cartesian2 = polar.to_cartesian3d();

        assert_eq!(approx(cartesian.x), cartesian2.x);
        assert_eq!(approx(cartesian.y), cartesian2.y);
        assert_eq!(approx(cartesian.z), cartesian2.z);
    }
}