use std::fmt;

pub enum Token {
  OpenParen,
  CloseParen,
  Identifier(String),
  Integer32(i32),
}

impl fmt::Show for Token {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Token::OpenParen => write!(formatter, "("),
      Token::CloseParen => write!(formatter, ")"),
      Token::Identifier(ref name) => write!(formatter, "{:}", name.as_slice()),
      Token::Integer32(ref val) => write!(formatter, "{}_i32", val)
    }
  }
}
