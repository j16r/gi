use ast::Node;
use environment::Environment;
use functions::default;

pub fn register(env: &mut Environment) {
    env.register("println".into(), default(println));
    env.register("dump".into(), default(dump));
    env.register("dumpln".into(), default(dumpln));
}

fn println(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Node::Cons(ref lhs_token, _) => println!("{}", lhs_token),
        _ => println!("{}", args),
    }
    box Node::Nil
}

fn dumpln(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Node::Cons(ref lhs_token, _) => println!("{:?}", lhs_token),
        _ => println!("{:?}", args),
    }
    box Node::Nil
}

fn dump(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    eprintln!("dump {:?}", args);
    match *args {
        box Node::Cons(ref lhs_token, _) => print!("{:?}", lhs_token),
        _ => print!("{:?}", args),
    }
    box Node::Nil
}
