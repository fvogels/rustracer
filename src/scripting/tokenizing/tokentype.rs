use crate::scripting::tokenizing::{Token, TokenizingError};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    Identifier,
    Integer,
    FloatingPointNumber,
}

impl TokenType {
    pub fn to_token(&self, string: String) -> Result<Token, TokenizingError> {
        match self {
            Self::LeftParenthesis => {
                if string != "(" {
                    Err(TokenizingError::LeftParenthesisError)
                } else {
                    Ok(Token::LeftParenthesis)
                }
            }
            Self::RightParenthesis => {
                if string != ")" {
                    Err(TokenizingError::RightParenthesisError)
                } else {
                    Ok(Token::RightParenthesis)
                }
            }
            Self::Identifier => Ok(Token::Identifier(string)),
            Self::Integer => {
                let n = string
                    .parse::<i64>()
                    .map_err(|e| TokenizingError::IntegerError(e))?;
                Ok(Token::Integer(n))
            }
            Self::FloatingPointNumber => {
                let n = string
                    .parse::<f64>()
                    .map_err(|e| TokenizingError::FloatingPointNumberError(e))?;
                Ok(Token::FloatingPointNumber(n))
            }
        }
    }
}
