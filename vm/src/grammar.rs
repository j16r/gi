use std::fmt;

pub enum Token {
    OpenParen,
    CloseParen,
    Bool(bool),
    Identifier(String),
    Integer32(i32),
    U8String(String),
}

impl fmt::Debug for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::OpenParen => write!(formatter, "("),
            Token::CloseParen => write!(formatter, ")"),
            Token::Bool(ref value) => write!(formatter, "{}", value),
            Token::Identifier(ref name) => write!(formatter, "{}", name),
            Token::Integer32(ref value) => write!(formatter, "{}_i32", value),
            Token::U8String(ref value) => write!(formatter, "\"{}\"", value)
        }
    }
}
