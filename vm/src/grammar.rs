use std::fmt;

pub enum Token {
  OpenParen,
  CloseParen,
  Identifier(String)
}

impl fmt::Show for Token {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      OpenParen => write!(formatter, "("),
      CloseParen => write!(formatter, ")"),
      Identifier(ref name) => write!(formatter, "{:s}", name.as_slice())
    }
  }
}
