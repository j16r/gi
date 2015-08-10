use ast::Node::{self, Nil, Atom, Cons, Lambda};
use ast::Value::{self, Bool};
use functions::{reserved, custom, FunctionTable, FunctionBody};
use lib;

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

fn def(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = env.eval(&first(args));
    let body = env.eval(&rest(args));

    if let box Node::Atom(ref name) = lhs {
        env.functions.insert(name.clone(), custom(body.clone()));
    } else {
        panic!("Expecting name to be a String, got {:?}", lhs);
    }
    body
}

fn lambda(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = first(args);
    let body = rest(args);
    box Node::Lambda(lhs, body)
}

fn register(env: &mut Environment) {
    env.register("first".into(), reserved(first_fn));
    env.register("rest".into(), reserved(rest_fn));
    env.register("cond".into(), reserved(cond));
    env.register("equal".into(), reserved(equal));
    env.register("quote".into(), reserved(quote));
    env.register("atom".into(), reserved(atom));
    env.register("cons".into(), reserved(cons));
    env.register("def".into(), reserved(def));
}

impl Environment {
    pub fn new() -> Box<Environment> {
        let mut environment = Environment {functions: FunctionTable::new()};

        register(&mut environment);
        lib::io::register(&mut environment);
        lib::math::register(&mut environment);

        box environment
    }

    pub fn register(&mut self, name: String, function: FunctionBody) {
        self.functions.insert(name, function);
    }

    pub fn eval(&mut self, token: &Box<Node>) -> Box<Node> {
        match *token {
            box Cons(ref head, ref tail) => {
                match *head {
                    box Cons(box Lambda(ref args, ref body), _) => {
                        self.invoke_lambda(args, body, tail)
                    },
                    box Atom(ref value) if value == "lambda" =>
                        lambda(self, tail),
                    box Atom(ref value) =>
                        self.invoke_function(value, tail),
                    _ => {
                        let head_result = self.eval(head);
                        let tail_result = self.eval(tail);
                        box Cons(head_result, tail_result)
                    }
                }
            },
            _ => token.clone()
        }
    }

    fn register_arg(&mut self, args: &Box<Node>, params: &Box<Node>) {
        let arg = match first(args) {
            box Atom(name) => name,
            _ => panic!("Argument definition is not an identifier")
        };
        let param = &first(params);
        self.register(arg, custom(param.clone()));

        match rest(params) {
            box Nil => return,
            _ => self.register_arg(&rest(args), &rest(param)),
        };
    }
    
    fn invoke_lambda(&mut self,
                     args: &Box<Node>,
                     body: &Box<Node>,
                     params: &Box<Node>) -> Box<Node> {
        self.register_arg(args, params);
        self.eval(body)
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
