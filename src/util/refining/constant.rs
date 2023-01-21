use super::Refine;

pub struct Constant<T: Clone> {
    value: T,
}

impl<T: Clone> Constant<T> {
    pub fn new(value: T) -> Self {
        Constant { value }
    }
}

impl<T: Clone> Refine<T> for Constant<T> {
    fn current(&self) -> T {
        self.value.clone()
    }

    fn refine(&mut self) {
        // NOP
    }
}
