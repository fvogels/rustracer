use std::rc::Rc;

use super::{interpreter::{Interpreter, InterpreterError}};

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    FloatingPointNumber(f64),
    Boolean(bool),
    List(Vec<Rc<Value>>),
    Symbol(String),
    Nil,
    NativeFunction(String, Rc<NativeFunction>)
}

pub type NativeFunction = dyn Fn(&mut Interpreter, &[Rc<Value>]) -> Result<Rc<Value>, InterpreterError>;

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
            Value::NativeFunction(_, _) => true,
            _ => false
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::FloatingPointNumber(x), Value::FloatingPointNumber(y)) => x == y,
            (Value::Symbol(x), Value::Symbol(y)) => x == y,
            (Value::List(xs), Value::List(ys)) => xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| x == y),
            (Value::Nil, Value::Nil) => true,
            _ => false
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(x) => f.debug_tuple("Boolean").field(x).finish(),
            Value::FloatingPointNumber(x) => f.debug_tuple("Float").field(x).finish(),
            Value::Integer(x) => f.debug_tuple("Float").field(x).finish(),
            Value::List(xs) => f.debug_list().entries(xs).finish(),
            Value::Nil => f.debug_tuple("Nil").finish(),
            Value::Symbol(x) => f.debug_tuple("Symbol").field(x).finish(),
            Value::NativeFunction(id, func) => f.debug_struct("NativeFunction").field("name", id).finish(),
        }
    }
}

pub mod creation {
    use super::Value;

    pub fn int(n: i64) -> Value {
        Value::Integer(n)
    }

    pub fn float(n: f64) -> Value {
        Value::FloatingPointNumber(n)
    }
}
