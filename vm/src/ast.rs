#[deriving(Clone)]
pub enum Token {
  Nil,
  True,
  False,
  OpenParen,
  CloseParen,
  Atom(String),
  Function,
  Lambda(Box<Token>, Box<Token>),
  Cons(Box<Token>, Box<Token>)
}
