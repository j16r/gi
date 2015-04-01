use std::fmt;

#[derive(Clone)]
pub enum Node {
  Nil,
  Atom(String),
  Bool(bool),
  Cons(Box<Node>, Box<Node>),
  Integer32(i32),
  U8String(String)
}

impl fmt::Debug for Node {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Node::Nil => write!(formatter, "Nil"),
      Node::Bool(ref value) => write!(formatter, "{}", value),
      Node::Atom(ref token) => write!(formatter, ":{}", token),
      Node::Cons(ref first, ref rest) => write!(formatter, "Cons({:?}, {:?})", first, rest),
      Node::Integer32(ref val) => write!(formatter, "{}_i32", val),
      Node::U8String(ref val) => write!(formatter, "\"{}\"", val)
    }
  }
}

impl fmt::Display for Node {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Node::Nil => write!(formatter, ""),
      Node::Atom(ref token) => write!(formatter, "{}", token),
      Node::Bool(ref value) => write!(formatter, "{}", value),
      Node::Cons(ref first, ref rest) => write!(formatter, "{}, {}", first, rest),
      Node::Integer32(ref val) => write!(formatter, "{}", val),
      Node::U8String(ref val) => write!(formatter, "{}", val)
    }
  }
}
