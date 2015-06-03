use std::collections::HashMap;
use ast::Node::{self, Nil, Atom, Cons};
use ast::Value::{self, Integer32, Bool};

type Builtin = fn (&mut Environment, &Box<Node>) -> Box<Node>;

pub struct Environment {
    functions: Box<HashMap<String, Box<Node>>>
}

fn first(args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => lhs_token.clone(),
        _ => panic!("Applied first to non Cons args {:?}", args)
    }
}

fn first_fn(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    first(args)
}

fn rest(args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(_, ref rhs_token) => rhs_token.clone(),
        _ => panic!("Applied rest to non Cons args {:?}", args)
    }
}

fn rest_fn(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    rest(args)
}

fn println(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => println!("{}", lhs_token),
        _ => println!("{}", args)
    }
    box Node::Nil
}

fn dumpln(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => println!("{:?}", lhs_token),
        _ => println!("{:?}", args)
    }
    box Node::Nil
}

fn dump(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => print!("{:?}", lhs_token),
        _ => print!("{:?}", args)
    }
    box Node::Nil
}

fn add(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value + *rhs_value))
        }
    }
    box Node::Nil
}

fn mul(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value * *rhs_value))
        }
    }
    box Node::Nil
}

fn cond(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    if let box Node::Value(Value::Bool(expression)) = first(args) {
        if expression {
            return rest(args);
        }
    } else {
        panic!("first argument to cond must be an Bool, got {:?}", args);
    }

    box Node::Nil
}

fn equal(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(ref lhs_value) = lhs {
        if let box Node::Value(ref rhs_value) = rhs {
            return box Node::Value(Value::Bool(*lhs_value == *rhs_value))
        } else {
            panic!("RHS? {:?}", rhs);
        }
    } else {
        panic!("LHS? {:?}", lhs);
    }

    box Node::Value(Value::Bool(false))
}

fn div(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value / *rhs_value))
        }
    }
    box Node::Nil
}

impl Environment {
    pub fn new() -> Box<Environment> {
        box Environment {
            functions: box HashMap::new()
        }
    }

    pub fn eval(&mut self, token: &Box<Node>) -> Box<Node> {
        match *token {
            box Cons(ref head, ref tail) => {
                let result = &self.eval(tail);
                match *head {
                    box Atom(ref value) => {
                        self.invoke_function(value, result)
                    },
                    _ => box Cons(self.eval(head), result.clone())
                }
            },
            _ => token.clone()
        }
    }

    fn invoke_function(&mut self, name: &String, args: &Box<Node>) -> Box<Node> {
        match &name[..] {
            "first" => first_fn(self, args),
            "rest" => rest_fn(self, args),
            "println" => println(self, args),
            "dumpln" => dumpln(self, args),
            "dump" => dump(self, args),
            "add" => add(self, args),
            "mul" => mul(self, args),
            "div" => div(self, args),
            "cond" => cond(self, args),
            "equal" => equal(self, args),
            _ => {
                match self.functions.get(name).cloned() {
                    Some(function) => self.eval(&function),
                    None => panic!("Tried to invoke function {} but there was none in scope", name)
                }
            }
        }
    }
}
