use std::collections::HashMap;
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
