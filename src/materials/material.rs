use crate::{imaging::color::Color, math::Vector, util::Refine};


pub type TraceFunction = Box<dyn Fn(&Vector<3>, f64) -> Color>;
pub type MaterialResult = Box<dyn Refine<Color>>;

pub trait Material {
    fn at(&self, direction: &Vector<3>, trace: TraceFunction) -> MaterialResult;
}
