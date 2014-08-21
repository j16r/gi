use vm::ast::{Token, Nil, Cons, Atom};

pub struct Environment {
  world: Box<Token>,
}

impl Environment {
  pub fn new() -> Box<Environment> {
    box Environment {
      world: box Cons(box Nil, box Nil),
    }
  }

  pub fn eval(&mut self, token: &Box<Token>) -> Box<Token> {
    box Atom("Hello".to_string())
  }
}
