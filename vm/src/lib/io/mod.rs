use ast::Node::{self, Nil, Cons};
use environment::Environment;
use functions::{FunctionTable, FunctionBody, default};

pub fn register(functions: &mut FunctionTable) {
    functions.insert("println".into(), default(println));
    functions.insert("dump".into(), default(dump));
    functions.insert("dumpln".into(), default(dumpln));
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
