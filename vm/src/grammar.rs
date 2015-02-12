use std::fmt;

pub enum Token {
  OpenParen,
  CloseParen,
  Identifier(String),
  Integer32(i32),
  U8String(String),
}

impl fmt::Debug for Token {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Token::OpenParen => write!(formatter, "("),
      Token::CloseParen => write!(formatter, ")"),
      Token::Identifier(ref name) => write!(formatter, "{}", name),
      Token::Integer32(ref val) => write!(formatter, "{}_i32", val),
      Token::U8String(ref val) => write!(formatter, "\"{}\"", val)
    }
  }
}
