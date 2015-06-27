use std::collections::HashMap;

use ast::Node::{self, Nil, Atom, Cons};
use ast::Value::{self, Bool};
use lib;
use functions::{reserved, FunctionTable, FunctionBody};

pub struct Environment {
    functions: FunctionTable
}

pub fn first(args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => lhs_token.clone(),
        _ => panic!("Applied first to non Cons args {:?}", args)
    }
}

fn first_fn(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    first(args)
}

pub fn rest(args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(_, ref rhs_token) => rhs_token.clone(),
        _ => panic!("Applied rest to non Cons args {:?}", args)
    }
}

fn rest_fn(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    rest(args)
}

fn cond(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    if let box Node::Value(Value::Bool(expression)) 
            = env.eval(&first(args)) {
        if expression {
            return env.eval(&rest(args));
        }
    } else {
        panic!("first argument to cond must be an Bool, got {:?}", args);
    }

    box Node::Nil
}

fn equal(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = env.eval(&first(args));
    let rhs = env.eval(&first(&rest(args)));

    box Node::Value(Value::Bool(*lhs == *rhs))
}

fn quote(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    first(args)
}

fn atom(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let result = match first(args) {
        box Node::Value(_) => true,
        _ => false
    };
    box Node::Value(Value::Bool(result))
}

fn cons(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = env.eval(&first(args));
    let rhs = env.eval(&rest(args));

    box Cons(lhs, rhs)
}

fn register(functions: &mut FunctionTable) {
    functions.insert("first".into(), reserved(first_fn));
    functions.insert("rest".into(), reserved(rest_fn));
    functions.insert("cond".into(), reserved(cond));
    functions.insert("equal".into(), reserved(equal));
    functions.insert("quote".into(), reserved(quote));
    functions.insert("atom".into(), reserved(atom));
    functions.insert("cons".into(), reserved(cons));
    functions.insert("def".into(), reserved(def));
}

impl Environment {
    pub fn new() -> Box<Environment> {
        let mut functions = FunctionTable::new();

        register(&mut functions);
        lib::io::register(&mut functions);
        lib::math::register(&mut functions);

        box Environment {functions: functions}
    }

    pub fn eval(&mut self, token: &Box<Node>) -> Box<Node> {
        match *token {
            box Cons(ref head, ref tail) => {
                match *head {
                    box Atom(ref value) =>
                        self.invoke_function(value, tail),
                    _ => {
                        let result = &self.eval(tail);
                        box Cons(self.eval(head), result.clone())
                    }
                }
            },
            _ => token.clone()
        }
    }

    fn function(&self, name: &String) -> FunctionBody {
        self.functions
            .get(name)
            .unwrap_or_else(|| panic!("Tried to invoke function {} but there was none in scope", name))
            .clone()
    }

    fn invoke_function(&mut self, name: &String, args: &Box<Node>) -> Box<Node> {
        match self.function(name) {
            FunctionBody::Reserved(ref body) => {
                let function = body.clone();
                body(self, args)
            },
            FunctionBody::Default(ref body) => {
                let result = &self.eval(args);
                body(self, result)
            },
            FunctionBody::Custom(ref body) => {
                let result = &self.eval(args);
                let function = box Node::Cons(
                    body.clone(), 
                    result.clone());
                self.eval(&function)
            }
        }
    }
}
