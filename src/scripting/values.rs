use std::rc::Rc;

use super::{interpreter::{Interpreter, InterpreterError}};

pub enum Value {
    Integer(i64),
    FloatingPointNumber(f64),
    Boolean(bool),
    List(Vec<Rc<Value>>),
    Symbol(String),
    Nil,
    NativeFunction(NativeFunction)
}

pub type NativeFunction = Rc<dyn Fn(&mut Interpreter, &[Rc<Value>]) -> Result<Rc<Value>, InterpreterError>>;

impl Value {
    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::FloatingPointNumber(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            _ => false
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_) => true,
            _ => false
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false
        }
    }

    pub fn is_native_function(&self) -> bool {
        match self {
            Value::NativeFunction(_) => true,
            _ => false
        }
    }
}