use std::rc::Rc;

use crate::{cameras::perspective::PerspectiveCamera, primitives::primitive::Primitive, lights::light::LightSource};

pub struct Scene {
    pub camera: PerspectiveCamera,
    pub root: Rc<dyn Primitive>,
    pub light_sources: Vec<Rc<dyn LightSource>>,
}
