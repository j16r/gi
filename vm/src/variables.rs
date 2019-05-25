use std::collections::HashMap;

use ast::Node;

pub type VariableTable = HashMap<String, Box<Node>>;
