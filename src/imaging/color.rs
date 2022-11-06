use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    rgb: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { rgb: [r, g, b] }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn r(&self) -> f64 {
        self.rgb[0]
    }

    pub fn g(&self) -> f64 {
        self.rgb[1]
    }

    pub fn b(&self) -> f64 {
        self.rgb[2]
    }

    pub fn to_byte_array(&self) -> [u8; 3] {
        fn clamp(c: f64) -> u8 {
            let c = (c * 256.0) as u8;
            min(255, max(0, c)) as u8
        }

        [clamp(self.r()), clamp(self.g()), clamp(self.b())]
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, c: Color) -> Self::Output {
        &self + &c
    }
}

impl std::ops::Add for &Color {
    type Output = Color;

    fn add(self, c: &Color) -> Self::Output {
        let r = self.r() + c.r();
        let g = self.g() + c.g();
        let b = self.b() + c.b();

        Color::new(r, g, b)
    }
}

impl std::ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        for i in 0..3 {
            self.rgb[i] += rhs.rgb[i];
        }
    }
}

impl std::ops::AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        for i in 0..3 {
            self.rgb[i] += rhs.rgb[i];
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let r = self.r() * rhs;
        let g = self.g() * rhs;
        let b = self.b() * rhs;

        Color::new(r, g, b)
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        &self * &rhs
    }
}

impl std::ops::Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        let r = self.r() * rhs.r();
        let g = self.g() * rhs.g();
        let b = self.b() * rhs.b();

        Color::new(r, g, b)
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl std::ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        let r = self.r() / rhs;
        let g = self.g() / rhs;
        let b = self.b() / rhs;

        Color::new(r, g, b)
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.rgb[i] /= rhs;
        }
    }
}
