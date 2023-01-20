mod linear;


pub trait Animation<T> {
    fn at(&self, t: f64) -> T;

    fn duration(&self) -> f64;
}

pub use linear::Linear;
