#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParenthesis,
    RightParenthesis,
    Identifier(String),
    Integer(i64),
    FloatingPointNumber(f64),
}
