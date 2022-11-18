use std::rc::Rc;

use crate::scripting::values::Value;

use super::environment::Environment;

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, ast: Rc<Value>) -> Result<Rc<Value>, InterpreterError> {
        match ast.as_ref() {
            Value::Integer(_) | Value::FloatingPointNumber(_) | Value::Boolean(_) | Value::Nil | Value::NativeFunction(_) => Ok(ast),
            Value::Symbol(ref id) => {
                self.environment.lookup(id)
            },
            Value::List(children) => {
                if children.is_empty() {
                    Ok(Rc::new(Value::Nil))
                } else {
                    let evaluated_children: Result<Vec<Rc<Value>>, _> = children.iter().map(|child| self.interpret(child.clone())).collect();
                    let evaluated_children = evaluated_children?;
                    let first = &evaluated_children[0];
                    let rest = &evaluated_children[1..];
                    match first.as_ref() {
                        Value::NativeFunction(ref native_function) => {
                            native_function(self, rest)
                        },
                        _ => Err(InterpreterError::CallingNonFunction)
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    Unbound(String),
    CallingNonFunction,
    MalformedLet,
    NonNumberInArithmeticOperation,
}
