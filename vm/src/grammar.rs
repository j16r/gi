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
      OpenParen => write!(formatter, "("),
      CloseParen => write!(formatter, ")"),
      Identifier(ref name) => write!(formatter, "{:}", name.as_slice()),
      Integer32(ref val) => write!(formatter, "{}_i32", val)
    }
  }
}
