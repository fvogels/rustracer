use std::rc::Rc;

use crate::scripting::values::Value;

use super::{environment::Environment, prelude::create_prelude};

pub struct Evaluator {
    pub environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        let environment = create_prelude();

        Evaluator { environment }
    }

    pub fn evaluate(&mut self, ast: Rc<Value>) -> Result<Rc<Value>, EvaluationError> {
        match ast.as_ref() {
            Value::Integer(_)
            | Value::FloatingPointNumber(_)
            | Value::Boolean(_)
            | Value::Nil
            | Value::NativeFunction(_, _) => Ok(ast),
            Value::Symbol(ref id) => self.environment.lookup(id),
            Value::List(children) => {
                if children.is_empty() {
                    Ok(Rc::new(Value::Nil))
                } else {
                    let evaluated_children: Result<Vec<Rc<Value>>, _> = children
                        .iter()
                        .map(|child| self.evaluate(child.clone()))
                        .collect();
                    let evaluated_children = evaluated_children?;
                    let first = &evaluated_children[0];
                    let rest = &evaluated_children[1..];
                    match first.as_ref() {
                        Value::NativeFunction(_, ref native_function) => {
                            native_function(self, rest)
                        }
                        _ => Err(EvaluationError::CallingNonFunction),
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum EvaluationError {
    Unbound(String),
    CallingNonFunction,
    MalformedLet,
    NonNumberInArithmeticOperation,
    InvalidNumberOfArguments,
    InvalidArgumentTypes,
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::scripting::values::creation::*;

    #[rstest]
    fn integer() {
        let mut evaluator = Evaluator::new();
        let ast = int(42);
        let expected = int(42);
        let actual = evaluator.evaluate(ast).unwrap();

        assert_eq!(expected, actual);
    }

    #[rstest]
    fn add_integers(
        #[values(
        vec![1],
        vec![1, 2],
        vec![-1, 2],
        vec![1, 2, 3, 4, 5],
        vec![42, 65, 18],
    )]
        mut values: Vec<i64>,
    ) {
        let mut evaluator = Evaluator::new();
        let expected = int(values.iter().sum());
        let mut elts: Vec<_> = values.into_iter().map(int).collect();
        elts.insert(0, symbol("+"));
        let ast = list(elts);
        let actual = evaluator.evaluate(ast).unwrap();

        assert_eq!(expected, actual);
    }
}
