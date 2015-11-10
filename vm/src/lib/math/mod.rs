use ast::Node::{self, Nil};
use ast::Value::Integer32;
use environment::{Environment, first, rest};
use functions::default;

pub fn register(env: &mut Environment) {
    env.register("add".into(), default(add));
    env.register("sub".into(), default(sub));
    env.register("mul".into(), default(mul));
    env.register("div".into(), default(div));
}

fn div(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value / *rhs_value));
        }
    }
    box Node::Nil
}

fn add(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value + *rhs_value));
        } else {
            panic!("can't add {:?}", rhs);
        }
    } else {
        panic!("can't add {:?}", lhs);
    }
    box Node::Nil
}

fn sub(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value - *rhs_value));
        } else {
            panic!("can't add {:?}", rhs);
        }
    } else {
        panic!("can't add {:?}", lhs);
    }
    box Node::Nil
}

fn mul(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value * *rhs_value));
        }
    }
    box Node::Nil
}
