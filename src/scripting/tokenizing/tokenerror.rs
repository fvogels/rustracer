use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum TokenizingError {
    LeftParenthesisError,
    RightParenthesisError,
    IntegerError(ParseIntError),
    FloatingPointNumberError(ParseFloatError),
    IncompleteToken,
}
