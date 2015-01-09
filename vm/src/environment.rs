use std::collections::HashMap;
use ast::Node;
use ast::Node::{Nil, Atom, Cons, Integer32};

type Builtin = fn (&mut Environment, &Box<Node>) -> Box<Node>;

pub struct Environment {
  functions: Box<HashMap<String, Box<Node>>>
}

fn println(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
  println!("{:}", args);
  box Node::Nil
}

fn add(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
  match *args {
    box Cons(ref lhs_token, ref tail) => {
      match *lhs_token {
        box Integer32(ref lhs_value) => {
          match *tail {
            box Cons(ref rhs_token, _) => {
              match *rhs_token {
                box Integer32(ref rhs_value) => {
                  box Integer32(*lhs_value + *rhs_value)
                },
                _ => panic!("second argument to add must be an Atom")
              }
            },
            _ => panic!("add only takes two arguments, got more")
          }
        },
        _ => panic!("first argument to add must be an Atom")
      }
    },
    _ => panic!("add requires two arguments")
  }
}

impl Environment {
  pub fn new() -> Box<Environment> {
    box Environment {
      functions: box HashMap::new()
    }
  }

  pub fn eval(&mut self, token: &Box<Node>) -> Box<Node> {
    println!("Evaluating... {}", token);
    match *token {
      box Cons(ref head, ref tail) => {
        let result = &self.eval(tail);
        match *head {
          box Integer32(_) => token.clone(),
          box Atom(ref value) => self.invoke_function(value, result),
          box Cons(_, _) => self.eval(head),
          _ => panic!("Non atom token {} in function position", head)
        }
      },
      _ => token.clone()
    }
  }

  fn invoke_function(&mut self, name: &String, args: &Box<Node>) -> Box<Node> {
    println!("Invoking {} with {}", name, args);
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
        let function = self.functions.get(name).cloned().unwrap();
        self.eval(&function)
      }
    }
  }
}
