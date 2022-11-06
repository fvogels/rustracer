pub struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEquation {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        QuadraticEquation { a, b, c }
    }

    pub fn solve(&self) -> Option<[f64; 2]> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            None
        } else {
            let sqrt_d = d.sqrt();

            let x1 = (-b - sqrt_d) / (2.0 * a);
            let x2 = (-b + sqrt_d) / (2.0 * a);

            if x1 <= x2 {
                Some([x1, x2])
            } else {
                Some([x2, x1])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 0, 0, Some([0.0, 0.0]))]
    #[case(1, 0, -1, Some([-1.0, 1.0]))]
    #[case(1, 0, -4, Some([-2.0, 2.0]))]
    #[case(1, -3, 2, Some([1.0, 2.0]))]
    #[case(1, 0, 1, None)]
    fn solve(#[case] a: i64, #[case] b: i64, #[case] c: i64, #[case] expected: Option<[f64; 2]>) {
        let eq = QuadraticEquation::new(a as f64, b as f64, c as f64);
        let actual = eq.solve();

        assert_eq!(expected, actual);
    }
}
