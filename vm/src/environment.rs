use std::collections::HashMap;
use ast::{Token, Nil, Cons, Atom};

pub struct Environment {
  world: Box<Token>,
  functions: Box<HashMap<String, Box<Token>>>
}

impl Environment {
  pub fn new() -> Box<Environment> {
    box Environment {
      world: box Cons(box Nil, box Nil),
      functions: box HashMap::new()
    }
  }

  pub fn eval(&mut self, token: &Box<Token>) -> Box<Token> {
    println!("Evaluating...");
    match *token {
      box Cons(ref head, ref tail) => {
        println!("Cons");
        match *head {
          box Atom(ref value) => {
            println!("Token");
            self.define_function(head, tail);
          },
          _ => fail!("Don't know what to do!")
        }
        self.eval(head);
      },
      _ => fail!("Don't know what to do!")
    }
    box Atom("Hello".to_string())
  }

  fn define_function(&mut self, name: &Box<Token>, args: &Box<Token>) {
    match *name {
      box Atom(ref value) => {
        println!("Defined function {:s}", value.to_string());
        self.functions.insert(value.to_owned(), args.clone());
      },
      _ => fail!("Tried to create function name with non Atom!")
    }
  }
}
