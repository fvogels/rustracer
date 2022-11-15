use std::num::{ParseFloatError, ParseIntError};

use crate::data::BufferedIterator;

use super::regex::{literal, Automaton, AutomatonBuilder};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    Identifier,
    Integer,
    FloatingPointNumber,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParenthesis,
    RightParenthesis,
    Identifier(String),
    Integer(i64),
    FloatingPointNumber(f64),
}

#[derive(Debug)]
pub enum TokenError {
    LeftParenthesisError,
    RightParenthesisError,
    IntegerError(ParseIntError),
    FloatingPointNumberError(ParseFloatError),
}

impl TokenType {
    pub fn to_token(&self, string: String) -> Result<Token, TokenError> {
        match self {
            Self::LeftParenthesis => {
                if string != "(" {
                    Err(TokenError::LeftParenthesisError)
                } else {
                    Ok(Token::LeftParenthesis)
                }
            }
            Self::RightParenthesis => {
                if string != ")" {
                    Err(TokenError::RightParenthesisError)
                } else {
                    Ok(Token::RightParenthesis)
                }
            }
            Self::Identifier => Ok(Token::Identifier(string)),
            Self::Integer => {
                let n = string
                    .parse::<i64>()
                    .map_err(|e| TokenError::IntegerError(e))?;
                Ok(Token::Integer(n))
            }
            Self::FloatingPointNumber => {
                let n = string
                    .parse::<f64>()
                    .map_err(|e| TokenError::FloatingPointNumberError(e))?;
                Ok(Token::FloatingPointNumber(n))
            }
        }
    }
}

pub struct Tokenizer<Loc: Copy + Clone, I: Iterator<Item = (char, Loc)>> {
    automaton: Automaton<TokenType>,
    input: BufferedIterator<I>,
}

impl<Loc: Copy + Clone, I: Iterator<Item = (char, Loc)>> Tokenizer<Loc, I> {
    pub fn new(input: I) -> Self {
        Tokenizer {
            automaton: Self::create_automaton(),
            input: BufferedIterator::new(input),
        }
    }

    fn create_automaton() -> Automaton<TokenType> {
        let mut builder = AutomatonBuilder::new();

        builder.add(literal('('), TokenType::LeftParenthesis);
        builder.add(literal(')'), TokenType::RightParenthesis);

        builder.eject()
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.input.current() {
                None => return,
                Some((ch, loc)) => {
                    if !Self::is_whitespace(ch) {
                        return;
                    } else {
                        self.input.next()
                    }
                }
            }
        }
    }

    fn is_whitespace(ch: char) -> bool {
        " \n\r".chars().any(|c| c == ch)
    }

    pub fn next_token(&mut self) -> Result<Option<(Token, Loc, Loc)>, TokenizerError> {
        self.skip_whitespace();

        match self.input.current() {
            None => Ok(None),
            Some((ch, start_location)) => {
                let mut last_location = start_location;
                let mut acc_string: String = String::from(ch);
                self.input.next();

                self.automaton.reset();
                self.automaton.feed(ch);

                loop {
                    match self.input.current() {
                        None => {
                            let token_type = self
                                .automaton
                                .finish()
                                .ok_or(TokenizerError::IncompleteToken)?;
                            let token = token_type
                                .to_token(acc_string)
                                .map_err(|e| TokenizerError::ConversionError(e))?;
                            let result = (token, start_location, last_location);
                            return Ok(Some(result));
                        }
                        Some((ch, loc)) => {
                            if self.automaton.feed(ch) {
                                acc_string.push(ch);
                                last_location = loc;
                                self.input.next();
                            } else {
                                let token_type = self
                                    .automaton
                                    .finish()
                                    .ok_or(TokenizerError::IncompleteToken)?;
                                let token = token_type
                                    .to_token(acc_string)
                                    .map_err(|e| TokenizerError::ConversionError(e))?;
                                let result = (token, start_location, last_location);
                                return Ok(Some(result));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenizerError {
    EndReached,
    IncompleteToken,
    ConversionError(TokenError),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    fn add_locs<'a>(string: &'a str) -> impl Iterator<Item = (char, usize)> + 'a {
        string.chars().enumerate().map(|(i, ch)| (ch, i))
    }

    #[rstest]
    fn parentheses() {
        let string = "()";
        let input = add_locs(string);
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            Some((Token::LeftParenthesis, 0, 0)),
            tokenizer.next_token().unwrap()
        );
        assert_eq!(
            Some((Token::RightParenthesis, 1, 1)),
            tokenizer.next_token().unwrap()
        );
        assert_eq!(None, tokenizer.next_token().unwrap());
    }

    #[rstest]
    fn parentheses_with_whitespace() {
        let string = " ( ) ";
        let input = add_locs(string);
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            Some((Token::LeftParenthesis, 1, 1)),
            tokenizer.next_token().unwrap()
        );
        assert_eq!(
            Some((Token::RightParenthesis, 3, 3)),
            tokenizer.next_token().unwrap()
        );
        assert_eq!(None, tokenizer.next_token().unwrap());
    }
}
