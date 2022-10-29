use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    rgb: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { rgb: [r, g, b] }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
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
        let r = self.r() + c.r();
        let g = self.g() + c.g();
        let b = self.b() + c.b();

        Color::new(r, g, b)
    }
}

impl std::ops::AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        for i in 0..3 {
            self.rgb[i] += rhs.rgb[i];
        }
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.rgb[i] /= rhs;
        }
    }
}
