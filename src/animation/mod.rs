mod linear;
mod time;
mod timeline;


pub trait Animation<T> {
    fn at(&self, t: TimeStamp) -> T;

    fn duration(&self) -> Duration;
}

pub use linear::LinearAnimation;
pub use time::{TimeStamp, Duration};
pub use timeline::TimeLine;
