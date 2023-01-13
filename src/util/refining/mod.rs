mod constant;


pub trait Refine<T> {
    fn current(&self) -> &T;

    fn eject(self) -> T;

    fn refine(&mut self);
}

pub use constant::Constant;