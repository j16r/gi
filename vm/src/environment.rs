use std::collections::HashMap;
use ast::{Token, Nil, Cons, Atom};
use parser;

type Builtin = fn (&mut Environment, &Box<Token>);

pub struct Environment {
  world: Box<Token>,
  builtins: Box<HashMap<String, Builtin>>,
  functions: Box<HashMap<String, Box<Token>>>
}

fn println(env: &mut Environment, args: &Box<Token>) {
  println!("{:s}", parser::dump(args));
}

impl Environment {
  pub fn new() -> Box<Environment> {
    let mut builtins = box HashMap::new();

    builtins.insert("println".to_string(), println);

    box Environment {
      world: box Cons(box Nil, box Nil),
      builtins: builtins,
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

        if *value == "println".to_string() {
          println(self, args);
        } else {
          let ref function = self.functions.get_copy(value);
          self.eval(function);
        }
      },
      _ => fail!("Tried to create function name with non Atom!")
    }
  }
}
