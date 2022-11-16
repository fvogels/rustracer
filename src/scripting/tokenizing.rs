use std::num::{ParseFloatError, ParseIntError};

use crate::data::BufferedIterator;

use super::regex::{
    alphanumeric, alternatives, character_class, floating_point, integer, kleene, letter, literal,
    one_or_more, sequence, Automaton, AutomatonBuilder, Regex,
};

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
        builder.add(integer(), TokenType::Integer);
        builder.add(floating_point(), TokenType::FloatingPointNumber);
        builder.add(Self::identifier_regex(), TokenType::Identifier);

        builder.eject()
    }

    fn identifier_regex() -> Regex {
        let identifier_char = alternatives(
            [character_class("+-*/%!@#$^&*|_<>=".chars()), alphanumeric()].into_iter(),
        );

        one_or_more(identifier_char)
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

    #[cfg(test)]
    fn check_with_locations(string: &str, expected_triples: &[(Token, usize, usize)]) {
        let input = add_locs(string);
        let mut tokenizer = Tokenizer::new(input);

        for expected_triple in expected_triples {
            let actual = tokenizer.next_token().unwrap();

            match actual {
                Some(actual_triple) => assert_eq!(*expected_triple, actual_triple),
                None => panic!("Missing tokens")
            }
        }

        assert_eq!(None, tokenizer.next_token().unwrap());
    }

    #[cfg(test)]
    fn check_without_locations(string: &str, expected_tokens: &[Token]) {
        let input = add_locs(string);
        let mut tokenizer = Tokenizer::new(input);

        for expected_token in expected_tokens {
            let actual = tokenizer.next_token().unwrap();

            match actual {
                Some((actual_token, _, _)) => assert_eq!(*expected_token, actual_token),
                None => panic!("Missing tokens")
            }
        }

        assert_eq!(None, tokenizer.next_token().unwrap());
    }

    #[rstest]
    fn parentheses() {
        let string = "()";
        let expected_tokens = [
            (Token::LeftParenthesis, 0, 0),
            (Token::RightParenthesis, 1, 1),
        ];

        check_with_locations(string, &expected_tokens);
    }

    #[rstest]
    fn parentheses_with_whitespace() {
        let string = " ( ) ";
        let expected_tokens = [
            (Token::LeftParenthesis, 1, 1),
            (Token::RightParenthesis, 3, 3),
        ];

        check_with_locations(string, &expected_tokens);
    }

    #[rstest]
    fn integers() {
        let string = "1 23 456 -10";
        let expected_tokens = [
            (Token::Integer(1), 0, 0),
            (Token::Integer(23), 2, 3),
            (Token::Integer(456), 5, 7),
            (Token::Integer(-10), 9, 11),
        ];

        check_with_locations(string, &expected_tokens);
    }

    #[rstest]
    fn floating_points() {
        let string = "1.0 12.3 999.7";
        let expected_tokens = [
            (Token::FloatingPointNumber(1.0), 0, 2),
            (Token::FloatingPointNumber(12.3), 4, 7),
            (Token::FloatingPointNumber(999.7), 9, 13),
        ];

        check_with_locations(string, &expected_tokens);
    }

    #[rstest]
    fn identifiers() {
        let string = "+ abc HELLO-WORLD";
        let expected_tokens = [
            (Token::Identifier("+".to_owned()), 0, 0),
            (Token::Identifier("abc".to_owned()), 2, 4),
            (Token::Identifier("HELLO-WORLD".to_owned()), 6, 16),
        ];

        check_with_locations(string, &expected_tokens);
    }

    #[rstest]
    fn mix() {
        let string = "(foo (+ 1.0 20) 8)";
        let expected_tokens = [
            Token::LeftParenthesis,
            Token::Identifier("foo".to_owned()),
            Token::LeftParenthesis,
            Token::Identifier("+".to_owned()),
            Token::FloatingPointNumber(1.0),
            Token::Integer(20),
            Token::RightParenthesis,
            Token::Integer(8),
            Token::RightParenthesis,
        ];

        check_without_locations(string, &expected_tokens);
    }
}
