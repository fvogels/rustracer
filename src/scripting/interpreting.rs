use std::rc::Rc;

use super::{
    evaluating::{EvaluationError, Evaluator},
    parsing::{Parser, ParsingError},
    tokenizing::{Tokenizer, TokenizingError},
    values::Value,
};

pub struct Interpreter {
    evaluator: Evaluator,
}

#[derive(Debug)]
pub enum InterpretingError {
    TokenizingError(TokenizingError),
    ParsingError(ParsingError),
    EvaluationError(EvaluationError),
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            evaluator: Evaluator::new(),
        }
    }

    pub fn interpret_string(&mut self, input: &str) -> Result<Rc<Value>, InterpretingError> {
        let input_iterator = input.chars().enumerate().map(Self::flip);
        let mut tokenizer = Tokenizer::new(input_iterator);
        let mut parser = Parser::new();

        while let Some((token, i, j)) = tokenizer
            .next_token()
            .map_err(InterpretingError::TokenizingError)?
        {
            parser
                .feed(&token)
                .map_err(InterpretingError::ParsingError)?;
        }

        let asts = parser.eject().map_err(InterpretingError::ParsingError)?;
        let mut last_result = Rc::new(Value::Nil);

        for ast in asts.into_iter() {
            last_result = self
                .evaluator
                .evaluate(ast)
                .map_err(InterpretingError::EvaluationError)?;
        }

        Ok(last_result)
    }

    fn flip<T1, T2>(pair: (T1, T2)) -> (T2, T1) {
        (pair.1, pair.0)
    }
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};

    #[cfg(test)]
    use super::*;

    #[fixture]
    fn interpreter() -> Interpreter {
        Interpreter::new()
    }

    #[rstest]
    fn addition(mut interpreter: Interpreter) {
        let input = "(+ 5 3)";
        let expected = Value::Integer(8);
        let actual = interpreter.interpret_string(input).unwrap();

        assert_eq!(expected, *actual);
    }

    #[rstest]
    fn subtraction(mut interpreter: Interpreter) {
        let input = "(- 5 3)";
        let expected = Value::Integer(2);
        let actual = interpreter.interpret_string(input).unwrap();

        assert_eq!(expected, *actual);
    }

    // #[rstest]
    // fn multiplication(mut interpreter: Interpreter) {
    //     let input = "(* 5 3)";
    //     let expected = Value::Integer(15);
    //     let actual = interpreter.interpret_string(input).unwrap();

    //     assert_eq!(expected, *actual);
    // }

    // #[rstest]
    // fn division(mut interpreter: Interpreter) {
    //     let input = "(/ 6 3)";
    //     let expected = Value::Integer(2);
    //     let actual = interpreter.interpret_string(input).unwrap();

    //     assert_eq!(expected, *actual);
    // }
}
