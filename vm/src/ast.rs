use std::fmt;

#[deriving(Clone)]
pub enum Node {
  Nil,
  Atom(String),
  Cons(Box<Node>, Box<Node>),
  Integer32(i32)
}

impl fmt::Show for Node {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Nil => write!(formatter, "Nil"),
      Atom(ref token) => write!(formatter, "{:}", token.as_slice()),
      Cons(ref first, ref rest) => write!(formatter, "Cons({}, {})", first, rest),
      Integer32(ref val) => write!(formatter, "{}_i32", val)
    }
  }
}
