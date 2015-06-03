use ast::Node::{self, Nil};
use ast::Value::Integer32;

use environment::{FunctionTable, Environment, first, rest};

pub fn register(functions: &mut FunctionTable) {
    functions.insert("add".into(), add);
    functions.insert("mul".into(), mul);
    functions.insert("div".into(), div);
}

fn div(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value / *rhs_value))
        }
    }
    box Node::Nil
}

fn add(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value + *rhs_value))
        }
    }
    box Node::Nil
}

fn mul(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let rhs = first(&rest(args));

    if let box Node::Value(Integer32(ref lhs_value)) = lhs {
        if let box Node::Value(Integer32(ref rhs_value)) = rhs {
            return box Node::Value(Integer32(*lhs_value * *rhs_value))
        }
    }
    box Node::Nil
}
