use std::collections::HashMap;
use ast::{Token, Nil, Cons, Atom};
use parser;

type Builtin = fn (&mut Environment, &Box<Token>) -> Box<Token>;

pub struct Environment {
  functions: Box<HashMap<String, Box<Token>>>
}

fn println(_: &mut Environment, args: &Box<Token>) -> Box<Token> {
  println!("{:s}", parser::dump(args));
  box Nil
}

fn add(_: &mut Environment, args: &Box<Token>) -> Box<Token> {
  match *args {
    box Cons(ref lhs_token, ref tail) => {
      match *lhs_token {
        box Atom(ref lhs_string) => {
          match *tail {
            box Cons(ref rhs_token, _) => {
              match *rhs_token {
                box Atom(ref rhs_string) => {
                  let lhs_value : Option<int> = from_str(lhs_string.as_slice());
                  let rhs_value : Option<int> = from_str(rhs_string.as_slice());
                  box Atom((lhs_value.unwrap() + rhs_value.unwrap()).to_string())
                },
                _ => fail!("second argument to add must be an Atom")
              }
            },
            _ => fail!("add only takes two arguments, got more")
          }
        },
        _ => fail!("first argument to add must be an Atom")
      }
    },
    _ => fail!("add requires two arguments")
  }
}

fn is_number_literal(string: &String) -> bool {
  string.as_slice().chars().all(|c| c >= '0' && c <= '9')
}

impl Environment {
  pub fn new() -> Box<Environment> {
    box Environment {
      functions: box HashMap::new()
    }
  }

  pub fn eval(&mut self, token: &Box<Token>) -> Box<Token> {
    println!("Evaluating... {}", parser::dump(token));
    match *token {
      box Cons(ref head, ref tail) => {
        let result = &self.eval(tail);
        match *head {
          box Atom(ref value) if is_number_literal(value) => token.clone(),
          box Atom(ref value) => self.invoke_function(value, result),
          box Cons(_, _) => self.eval(head),
          _ => fail!("Non atom token {} in function position", parser::dump(head))
        }
      },
      _ => token.clone()
    }
  }

  fn invoke_function(&mut self, name: &String, args: &Box<Token>) -> Box<Token> {
    println!("Invoking {} with {}", name, parser::dump(args));
    match name.as_slice() {
      "println" => {
        let result = &println(self, args);
        self.eval(result)
      },
      "add" => {
        let result = &add(self, args);
        self.eval(result)
      },
      _ => {
        let function = &self.functions.get_copy(name);
        self.eval(function)
      }
    }
  }
}
