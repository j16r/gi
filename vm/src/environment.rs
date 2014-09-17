use std::collections::HashMap;
use ast::{Token, Nil, Cons, Atom};

pub struct Environment {
  world: Box<Token>,
  functions: Box<HashMap<String, Box<Token>>>
}

impl Environment {
  pub fn new() -> Box<Environment> {
    let mut functions = box HashMap::new();

    functions.insert("println".to_string(), box Nil);

    box Environment {
      world: box Cons(box Nil, box Nil),
      functions: functions
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
            self.invoke_function(head, tail);
          },
          _ => fail!("Don't know what to do!")
        }
        self.eval(head);
      },
      box Nil => println!("no-op"),
      _ => fail!("Don't know what to do!")
    }
    box Atom("Hello".to_string())
  }

  fn invoke_function(&mut self, name: &Box<Token>, args: &Box<Token>) {
    match *name {
      box Atom(ref value) => {
        println!("Invoking function {:s}", value.to_string());
        let ref function = self.functions.get_copy(value);
        self.eval(function);
      },
      _ => fail!("Tried to create function name with non Atom!")
    }
  }
}
