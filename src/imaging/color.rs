#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    rgb: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            rgb: [r, g, b]
        }
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
