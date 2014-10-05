use std::fmt;

#[deriving(Clone)]
pub enum Node {
  Nil,
  Atom(String),
  Cons(Box<Node>, Box<Node>)
}

impl fmt::Show for Node {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Nil => write!(formatter, "Nil"),
      Atom(ref token) => write!(formatter, "{:s}", token.as_slice()),
      Cons(ref first, ref rest) => write!(formatter, "Cons({}, {})", first, rest)
    }
  }
}
