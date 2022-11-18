use std::rc::Rc;

use crate::data::Either;

use super::{environment::Environment, values::Value, interpreter::InterpreterError};

pub fn create_prelude() -> Environment {
    let mut environment = Environment::new();

    environment
}

// fn r#let(interpreter: &mut Interpreter, arguments: &[Rc<Value>]) -> Result<Rc<Value>, InterpreterError> {
//     match arguments {
//         [bindings, body] => {

//             interpreter.interpret(body.clone())
//         },
//         _ => {
//             Err(InterpreterError::NativeFunctionError(NativeFunctionError::InvalidLet))
//         }
//     }
// }

// fn addition(interpreter: &mut Interpreter, arguments: &[Rc<Value>]) -> Result<Rc<Value>, InterpreterError> {

// }

fn homogenize_numbers(values: &[Rc<Value>]) -> Result<Either<Vec<i64>, Vec<f64>>, InterpreterError> {
    let mut result: Either<Vec<i64>, Vec<f64>> = Either::Left(Vec::new());

    for value in values {
        match (value.as_ref(), &mut result) {
            (Value::Integer(n), Either::Left(vec)) => vec.push(*n),
            (Value::FloatingPointNumber(n), Either::Left(vec)) => {
                let mut converted: Vec<f64> = vec.iter().map(|&k| k as f64).collect();
                converted.push(*n);
                result = Either::Right(converted)
            },
            (Value::Integer(n), Either::Right(vec)) => vec.push(n.clone() as f64),
            (Value::FloatingPointNumber(n), Either::Right(vec)) => vec.push(*n),
            _ => return Err(InterpreterError::NonNumberInArithmeticOperation),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn homogenize_single_i64() {
        let values = [ Value::Integer(5) ].map(|v| Rc::new(v));
        let expected = Either::Left(vec![ 5 ]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_single_f64() {
        let values = [ Value::FloatingPointNumber(7.8) ].map(|v| Rc::new(v));
        let expected = Either::Right(vec![ 7.8 ]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_multiple_i64() {
        let values = [ Value::Integer(5), Value::Integer(10) ].map(|v| Rc::new(v));
        let expected = Either::Left(vec![ 5, 10 ]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_multiple_f64() {
        let values = [ Value::FloatingPointNumber(7.8), Value::FloatingPointNumber(9.1) ].map(|v| Rc::new(v));
        let expected = Either::Right(vec![ 7.8, 9.1 ]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_mix1() {
        let values = [ Value::FloatingPointNumber(7.8), Value::FloatingPointNumber(9.1), Value::Integer(5) ].map(|v| Rc::new(v));
        let expected = Either::Right(vec![ 7.8, 9.1, 5.0 ]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_mix2() {
        let values = [ Value::Integer(5), Value::FloatingPointNumber(7.8), Value::FloatingPointNumber(9.1) ].map(|v| Rc::new(v));
        let expected = Either::Right(vec![ 5.0, 7.8, 9.1 ]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }
}
