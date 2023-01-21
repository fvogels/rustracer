use std::rc::Rc;

use crate::{
    cameras::perspective::PerspectiveCamera, lights::light::LightSource,
    primitives::Primitive,
};

pub struct Scene {
    pub camera: PerspectiveCamera,
    pub root: Rc<dyn Primitive>,
    pub light_sources: Vec<Rc<dyn LightSource>>,
}
