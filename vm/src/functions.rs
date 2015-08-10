use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use environment::Environment;
use ast::Node;

pub type InternalFunctionBody = fn (&mut Environment, &Box<Node>) -> Box<Node>;

#[derive(Clone)]
pub enum FunctionBody {
    Reserved(Rc<InternalFunctionBody>),
    Default(Rc<InternalFunctionBody>),
    Custom(Box<Node>)
}

pub type FunctionTable = HashMap<String, FunctionBody>;

pub fn reserved(body: InternalFunctionBody) -> FunctionBody {
    FunctionBody::Reserved(Rc::new(body))
}

pub fn default(body: InternalFunctionBody) -> FunctionBody {
    FunctionBody::Default(Rc::new(body))
}

pub fn custom(body: Box<Node>) -> FunctionBody {
    FunctionBody::Custom(body)
}

impl fmt::Debug for FunctionBody {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FunctionBody::Reserved(_) =>
                write!(formatter, "fn(reserved)"),
            FunctionBody::Default(_) =>
                write!(formatter, "fn(default)"),
            FunctionBody::Custom(_) =>
                write!(formatter, "fn")
        }
    }
}
