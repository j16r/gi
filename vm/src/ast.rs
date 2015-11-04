use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Integer32(i32),
    U8String(String),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Node {
    Nil,
    Atom(String),
    Cons(Box<Node>, Box<Node>),
    Value(Value),
    Lambda(Box<Node>, Box<Node>),
}

impl fmt::Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(ref value) => write!(formatter, "{}", value),
            Value::Integer32(ref val) => write!(formatter, "{}_i32", val),
            Value::U8String(ref val) => write!(formatter, "\"{}\"", val),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(ref value) => write!(formatter, "{}", value),
            Value::Integer32(ref val) => write!(formatter, "{}", val),
            Value::U8String(ref val) => write!(formatter, "{}", val),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Nil => write!(formatter, "Nil"),
            Node::Atom(ref token) => write!(formatter, ":{}", token),
            Node::Cons(ref first, ref rest) => write!(formatter, "({:?}, {:?})", first, rest),
            Node::Value(ref val) => write!(formatter, "{:?}", val),
            Node::Lambda(ref args, ref body) => write!(formatter, "'({:?}, {:?})", args, body),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Nil => write!(formatter, ""),
            Node::Atom(ref token) => write!(formatter, "{}", token),
            Node::Cons(ref first, ref rest) => write!(formatter, "{}, {}", first, rest),
            Node::Value(ref val) => write!(formatter, "{}", val),
            Node::Lambda(ref args, ref body) => write!(formatter, "'({}, {})", args, body),
        }
    }
}
