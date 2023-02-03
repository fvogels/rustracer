mod constant;

pub use constant::Constant;


pub trait Refine<T> {
    fn current(&self) -> T;

    fn refine(&mut self);
}

