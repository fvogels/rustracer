use std::rc::Rc;

use crate::{data::Either, scripting::values::NativeFunction};

use super::{
    environment::Environment,
    evaluating::{EvaluationError, Evaluator},
    values::Value,
};

pub fn create_prelude() -> Environment {
    fn native_function<
        F: Fn(&mut Evaluator, &[Rc<Value>]) -> Result<Rc<Value>, EvaluationError> + 'static,
    >(
        id: &str,
        f: F,
    ) -> Rc<Value> {
        Rc::new(Value::NativeFunction(String::from(id), Rc::new(f)))
    }

    let mut environment = Environment::new();

    environment.bind(String::from("+"), native_function("+", lib::addition));
    environment.bind(String::from("-"), native_function("-", lib::subtraction));
    environment.bind(String::from("*"), native_function("*", lib::multiplication));

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

mod lib {
    use super::*;

    pub fn addition(
        _interpreter: &mut Evaluator,
        arguments: &[Rc<Value>],
    ) -> Result<Rc<Value>, EvaluationError> {
        if arguments.is_empty() {
            Err(EvaluationError::InvalidNumberOfArguments)
        } else {
            let mut result = arguments[0].as_ref().clone();

            for argument in arguments[1..].iter() {
                match (result, argument.as_ref()) {
                    (Value::Integer(a), Value::Integer(b)) => result = Value::Integer(a + b),
                    (Value::Integer(a), Value::FloatingPointNumber(b)) => {
                        result = Value::FloatingPointNumber(a as f64 + b)
                    }
                    (Value::FloatingPointNumber(a), Value::FloatingPointNumber(b)) => {
                        result = Value::FloatingPointNumber(a + b)
                    }
                    (Value::FloatingPointNumber(a), Value::Integer(b)) => {
                        result = Value::FloatingPointNumber(a + (*b as f64))
                    }
                    _ => return Err(EvaluationError::InvalidArgumentTypes),
                }
            }

            Ok(Rc::new(result))
        }
    }

    pub fn subtraction(
        _interpreter: &mut Evaluator,
        arguments: &[Rc<Value>],
    ) -> Result<Rc<Value>, EvaluationError> {
        if arguments.is_empty() {
            Err(EvaluationError::InvalidNumberOfArguments)
        } else if arguments.len() == 1 {
            match arguments[0].as_ref() {
                Value::Integer(a) => {
                    let result = Value::Integer(-a);
                    Ok(Rc::new(result))
                }
                Value::FloatingPointNumber(a) => {
                    let result = Value::FloatingPointNumber(-a);
                    Ok(Rc::new(result))
                }
                _ => Err(EvaluationError::InvalidArgumentTypes),
            }
        } else {
            let mut result = arguments[0].as_ref().clone();

            for argument in arguments[1..].iter() {
                match (result, argument.as_ref()) {
                    (Value::Integer(a), Value::Integer(b)) => result = Value::Integer(a - b),
                    (Value::Integer(a), Value::FloatingPointNumber(b)) => {
                        result = Value::FloatingPointNumber(a as f64 - b)
                    }
                    (Value::FloatingPointNumber(a), Value::FloatingPointNumber(b)) => {
                        result = Value::FloatingPointNumber(a - b)
                    }
                    (Value::FloatingPointNumber(a), Value::Integer(b)) => {
                        result = Value::FloatingPointNumber(a - (*b as f64))
                    }
                    _ => return Err(EvaluationError::InvalidArgumentTypes),
                }
            }

            Ok(Rc::new(result))
        }
    }

    pub fn multiplication(
        _interpreter: &mut Evaluator,
        arguments: &[Rc<Value>],
    ) -> Result<Rc<Value>, EvaluationError> {
        if arguments.is_empty() {
            Err(EvaluationError::InvalidNumberOfArguments)
        } else {
            let mut result = arguments[0].as_ref().clone();

            for argument in arguments[1..].iter() {
                match (result, argument.as_ref()) {
                    (Value::Integer(a), Value::Integer(b)) => result = Value::Integer(a * b),
                    (Value::Integer(a), Value::FloatingPointNumber(b)) => {
                        result = Value::FloatingPointNumber(a as f64 * b)
                    }
                    (Value::FloatingPointNumber(a), Value::FloatingPointNumber(b)) => {
                        result = Value::FloatingPointNumber(a * b)
                    }
                    (Value::FloatingPointNumber(a), Value::Integer(b)) => {
                        result = Value::FloatingPointNumber(a * (*b as f64))
                    }
                    _ => return Err(EvaluationError::InvalidArgumentTypes),
                }
            }

            Ok(Rc::new(result))
        }
    }
}

fn homogenize_numbers(values: &[Rc<Value>]) -> Result<Either<Vec<i64>, Vec<f64>>, EvaluationError> {
    let mut result: Either<Vec<i64>, Vec<f64>> = Either::Left(Vec::new());

    for value in values {
        match (value.as_ref(), &mut result) {
            (Value::Integer(n), Either::Left(vec)) => vec.push(*n),
            (Value::FloatingPointNumber(n), Either::Left(vec)) => {
                let mut converted: Vec<f64> = vec.iter().map(|&k| k as f64).collect();
                converted.push(*n);
                result = Either::Right(converted)
            }
            (Value::Integer(n), Either::Right(vec)) => vec.push(n.clone() as f64),
            (Value::FloatingPointNumber(n), Either::Right(vec)) => vec.push(*n),
            _ => return Err(EvaluationError::NonNumberInArithmeticOperation),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::scripting::values::creation::*;

    #[rstest]
    fn homogenize_single_i64() {
        let values = [Value::Integer(5)].map(|v| Rc::new(v));
        let expected = Either::Left(vec![5]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_single_f64() {
        let values = [Value::FloatingPointNumber(7.8)].map(|v| Rc::new(v));
        let expected = Either::Right(vec![7.8]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_multiple_i64() {
        let values = [Value::Integer(5), Value::Integer(10)].map(|v| Rc::new(v));
        let expected = Either::Left(vec![5, 10]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_multiple_f64() {
        let values = [
            Value::FloatingPointNumber(7.8),
            Value::FloatingPointNumber(9.1),
        ]
        .map(|v| Rc::new(v));
        let expected = Either::Right(vec![7.8, 9.1]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_mix1() {
        let values = [
            Value::FloatingPointNumber(7.8),
            Value::FloatingPointNumber(9.1),
            Value::Integer(5),
        ]
        .map(|v| Rc::new(v));
        let expected = Either::Right(vec![7.8, 9.1, 5.0]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[rstest]
    fn homogenize_mix2() {
        let values = [
            Value::Integer(5),
            Value::FloatingPointNumber(7.8),
            Value::FloatingPointNumber(9.1),
        ]
        .map(|v| Rc::new(v));
        let expected = Either::Right(vec![5.0, 7.8, 9.1]);
        let actual = homogenize_numbers(&values).unwrap();

        assert_eq!(expected, actual)
    }

    #[fixture]
    fn interpreter() -> Evaluator {
        Evaluator::new()
    }

    #[rstest]
    #[case(&[int(0)], int(0))]
    #[case(&[int(1)], int(1))]
    #[case(&[int(1), int(2)], int(3))]
    #[case(&[int(1), int(2), int(3)], int(6))]
    #[case(&[int(1), int(2), int(4)], int(7))]
    #[case(&[float(1.0), float(2.0), float(3.0)], float(6.0))]
    #[case(&[int(1), float(2.0), float(3.0)], float(6.0))]
    fn addition(
        mut interpreter: Evaluator,
        #[case] arguments: &[Rc<Value>],
        #[case] expected: Rc<Value>,
    ) {
        let actual = lib::addition(&mut interpreter, &arguments).unwrap();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(&[int(0)], int(0))]
    #[case(&[int(1)], int(-1))]
    #[case(&[int(2), int(1)], int(1))]
    #[case(&[int(3), int(2), int(1)], int(0))]
    #[case(&[float(3.0), int(2), int(1)], float(0.0))]
    fn subtraction(
        mut interpreter: Evaluator,
        #[case] arguments: &[Rc<Value>],
        #[case] expected: Rc<Value>,
    ) {
        let actual = lib::subtraction(&mut interpreter, &arguments).unwrap();

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(&[int(0)], int(0))]
    #[case(&[int(1)], int(1))]
    #[case(&[int(1), int(2)], int(2))]
    #[case(&[int(1), int(2), int(4)], int(8))]
    #[case(&[float(1.0), float(2.0), float(3.0)], float(6.0))]
    #[case(&[int(1), float(2.0), float(3.0)], float(6.0))]
    fn multiplication(
        mut interpreter: Evaluator,
        #[case] arguments: &[Rc<Value>],
        #[case] expected: Rc<Value>,
    ) {
        let actual = lib::multiplication(&mut interpreter, &arguments).unwrap();

        assert_eq!(expected, actual);
    }
}
