struct Color {
    rgb: [f64; 3],
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            rgb: [r, g, b]
        }
    }

    fn r(&self) -> f64 {
        self.rgb[0]
    }

    fn g(&self) -> f64 {
        self.rgb[1]
    }

    fn b(&self) -> f64 {
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
