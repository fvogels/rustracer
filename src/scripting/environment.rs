use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{evaluating::EvaluationError, values::Value};

type Bindings = HashMap<String, Rc<Value>>;

#[derive(Clone)]
pub struct Environment {
    top_frame: Rc<RefCell<Frame>>,
}

struct Frame {
    bindings: Bindings,
    parent: Option<Rc<RefCell<Frame>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            top_frame: Rc::new(RefCell::new(Frame::new(None))),
        }
    }

    pub fn lookup(&self, id: &String) -> Result<Rc<Value>, EvaluationError> {
        self.top_frame.borrow_mut().lookup(id)
    }

    pub fn bind(&mut self, id: String, value: Rc<Value>) {
        self.top_frame.borrow_mut().bind(id, value)
    }
}

impl Frame {
    pub fn new(parent: Option<Rc<RefCell<Frame>>>) -> Self {
        let bindings = HashMap::new();

        Frame { bindings, parent }
    }

    pub fn lookup(&self, id: &String) -> Result<Rc<Value>, EvaluationError> {
        match self.bindings.get(id) {
            None => self
                .parent
                .as_ref()
                .ok_or_else(|| EvaluationError::Unbound(id.clone()))
                .and_then(|p| p.as_ref().borrow().lookup(id)),
            Some(value) => Ok(value.clone()),
        }
    }

    pub fn bind(&mut self, id: String, value: Rc<Value>) {
        self.bindings.insert(id, value);
    }
}
