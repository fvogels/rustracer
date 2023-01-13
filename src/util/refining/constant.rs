use super::Refine;

pub struct Constant<T> {
    value: T,
}

impl<T> Constant<T> {
    pub fn new(value: T) -> Self {
        Constant { value }
    }
}

impl<T> Refine<T> for Constant<T> {
    fn current(&self) -> &T {
        &self.value
    }

    fn eject(self) -> T {
        self.value
    }

    fn refine(&mut self) {
        // NOP
    }
}
