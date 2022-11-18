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
