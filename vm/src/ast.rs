#[deriving(Clone)]
pub enum Node {
  Nil,
  Atom(String),
  Cons(Box<Node>, Box<Node>)
}
