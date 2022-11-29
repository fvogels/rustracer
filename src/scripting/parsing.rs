use std::rc::Rc;

use super::{values::Value, tokenizing::Token};

pub struct Parser {
    stack: Vec<Vec<Rc<Value>>>
}

#[derive(Debug)]
pub enum ParsingError {
    StackUnderflow,
    UnfinishedList,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            stack: vec![ Vec::new() ]
        }
    }

    pub fn feed(&mut self, token: &Token) -> Result<(), ParsingError> {
        match token {
            Token::FloatingPointNumber(n) => {
                self.push(Value::FloatingPointNumber(*n))?;
            }
            Token::Identifier(id) => {
                self.push(Value::Symbol(id.clone()))?;
            }
            Token::Integer(n) => {
                self.push(Value::Integer(*n))?;
            }
            Token::LeftParenthesis => {
                self.stack.push(Vec::new());
            }
            Token::RightParenthesis => {
                let elts = self.pop()?;
                let list = Value::List(elts);
                self.push(list)?;
            }
        }

        Ok(())
    }

    fn top(&mut self) -> Result<&mut Vec<Rc<Value>>, ParsingError> {
        self.stack.last_mut().ok_or(ParsingError::StackUnderflow)
    }

    fn push(&mut self, value: Value) -> Result<(), ParsingError> {
        self.top()?.push(Rc::new(value));
        Ok(())
    }

    fn pop(&mut self) -> Result<Vec<Rc<Value>>, ParsingError> {
        self.stack.pop().ok_or(ParsingError::StackUnderflow)
    }

    pub fn eject(mut self) -> Result<Vec<Rc<Value>>, ParsingError> {
        if self.stack.len() == 1 {
            self.stack.pop().ok_or(ParsingError::StackUnderflow)
        } else {
            Err(ParsingError::UnfinishedList)
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn parse_single_symbol() {
        let id = String::from("abc");
        let mut parser = Parser::new();
        parser.feed(&Token::Identifier(id.clone()));

        let mut result = parser.eject().unwrap();
        assert_eq!(1, result.len());
        let actual = result.pop().unwrap();
        assert_eq!(&Value::Symbol(id), actual.as_ref());
    }

    #[rstest]
    fn parse_list() {
        let mut parser = Parser::new();
        parser.feed(&Token::LeftParenthesis);
        parser.feed(&Token::Integer(1));
        parser.feed(&Token::Integer(2));
        parser.feed(&Token::Integer(3));
        parser.feed(&Token::RightParenthesis);

        let mut result = parser.eject().unwrap();
        assert_eq!(1, result.len());
        let actual = result.pop().unwrap();

        let expected = Value::List(vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ]);
        assert_eq!(&expected, actual.as_ref());
    }
}