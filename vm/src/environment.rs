use ast::Node::{self, Nil, Atom, Cons, Lambda, Function, FunctionApplication, Let};
use ast::Value::Bool;
use functions::{reserved, custom, FunctionTable, FunctionBody};
use variables::{VariableTable};
use lib;

pub struct Environment {
    functions: FunctionTable,
    variables: VariableTable,
}

pub fn first(args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => lhs_token.clone(),
        _ => panic!("Applied first to non Cons args {:?}", args),
    }
}

fn first_fn(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    first(args)
}

pub fn rest(args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(_, ref rhs_token) => rhs_token.clone(),
        _ => panic!("Applied rest to non Cons args {:?}", args),
    }
}

fn rest_fn(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    rest(args)
}

fn cond(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    if let box Node::Value(Bool(expression)) = env.eval(&first(args)) {
        let clauses = &rest(args);
        if expression {
            eprintln!("  expression is true!");
            let result = env.eval(&first(clauses));
            eprintln!("  returning {:?}", result);
            return result;
        } else {
            eprintln!("  expression is false!");
            let result = env.eval(&rest(clauses));
            eprintln!("  returning {:?}", result);
            return first(&result);
        }
    } else {
        panic!("first argument to cond must be an Bool, got {:?}", args);
    }
}

fn equal(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let lhs = env.eval(&first(args));
    let rhs = env.eval(&first(&rest(args)));

    eprintln!("  {:?} == {:?}", lhs, rhs);

    box Node::Value(Bool(*lhs == *rhs))
}

fn quote(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    first(args)
}

fn atom(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    let result = match first(args) {
        box Node::Value(_) => true,
        _ => false,
    };
    box Node::Value(Bool(result))
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

    eprintln!("Created lambda {:?} => {:?}", lhs, body);

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
        let mut environment = Environment {
            functions: FunctionTable::new(),
            variables: VariableTable::new(),
        };

        register(&mut environment);
        lib::io::register(&mut environment);
        lib::math::register(&mut environment);

        box environment
    }

    pub fn register(&mut self, name: String, function: FunctionBody) {
        self.functions.insert(name, function);
    }

    pub fn define_function(&mut self, name: &String, body: &Box<Node>) -> Box<Node> {
        self.functions.insert(name.clone(), custom(body.clone()));
        body.clone()
    }

    pub fn eval(&mut self, token: &Box<Node>) -> Box<Node> {
        eprintln!("Evaluating {:?}", token);
        match *token {
            box Function(ref name, _, ref body) => self.define_function(name, body),
            box FunctionApplication(ref ident, ref args) => self.invoke_function(ident, args),
            box Let(ref name, ref value) => self.register_label(name, value),
            box Atom(ref name) => self.load_variable(name), // , tail),
            box Cons(ref head, ref tail) => {
                match *head {
                    box Cons(box Lambda(ref args, ref body), _) => {
                        self.execute_lambda(args, tail, body)
                    },
                    box Atom(ref name) if name == "lambda" => lambda(self, tail),
                    _ => {
                        eprintln!("eval else...");
                        let head_result = self.eval(head);
                        let tail_result = self.eval(tail);
                        box Cons(head_result, tail_result)
                    }
                }
            }
            _ => token.clone(),
        }
    }

    fn register_label(&mut self, name: &Box<Node>, value: &Box<Node>) -> Box<Node> {
        if let &box Node::Atom(ref name) = name {
            eprintln!("Registered {:?} = {:?}", name, value);
            self.functions.insert(name.clone(), custom(value.clone()));
        } else {
            panic!("Expecting name to be a String, got {:?}", name);
        }
        value.clone()
    }

    // insert_parameters replaces each instance of the atom from args with its corresponding parameter
    fn insert_parameters(&mut self, token: &Box<Node>, args: &Box<Node>, params: &Box<Node>) -> Box<Node> {
        let arg = match first(args) {
            box Atom(name) => name,
            _ => panic!("Argument definition is not an identifier"),
        };
        let param = &first(params);

        let new_body = match *token {
            box Atom(ref value) if *value == arg => param.clone(),
            box Cons(ref head, ref tail) => {
                let new_head = self.insert_parameters(head, args, params);
                let new_tail = self.insert_parameters(tail, args, params);
                box Cons(new_head, new_tail)
            },
            _ => token.clone(),
        };

        match rest(args) {
            box Nil => return new_body,
            _ => self.insert_parameters(&new_body, &rest(args), &rest(params))
        }
    }

    // execute_lambda performs an eval over the body of the lambda function, splicing in the
    // parameters
    //
    // args = the identifiers of expected arguments, e.g:
    // lambda (value)
    // then args = [value]
    //
    // params = the values passed in, in order, i.e. if lambda is called like so:
    // (lambda 1)
    // then params = [1]
    //
    // body is the actual code to execute
    fn execute_lambda(&mut self,
                      args: &Box<Node>,
                      params: &Box<Node>,
                      body: &Box<Node>) -> Box<Node> {
        eprintln!("execing lambda\n\targs: {:?}\n\tparams: {:?}\n\tbody: {:?}", args, params, body);
        let new_body = self.insert_parameters(body, args, params);
        self.eval(&new_body)
    }

    fn function(&self, name: &String) -> FunctionBody {
        self.functions
            .get(name)
            .unwrap_or_else(|| {
                panic!("Tried to invoke function {} but there was none in scope",
                       name)
            })
            .clone()
    }

    fn load_variable(&mut self, name: &String) -> Box<Node> {
        eprintln!("Looking up {:?}...", name);
        self.variables
            .get(name)
            .unwrap_or_else(|| {
                panic!("Tried to use variable {} but there was none in scope",
                       name)
            })
            .clone()
    }

    fn invoke_function(&mut self, name: &String, args: &Box<Node>) -> Box<Node> {
        let result = match self.function(name) {
            FunctionBody::Reserved(ref body) => {
                eprintln!("Invoking reserved function {:?} {:?}", name, args);
                body(self, args)
            }
            FunctionBody::Default(ref body) => {
                eprintln!("Invoking default function {:?} {:?}", name, args);
                let result = &self.eval(args);
                body(self, result)
            }
            FunctionBody::Custom(ref body) => {
                eprintln!("Invoking custom function {:?} {:?}", name, args);
                let result = &self.eval(args);
                let function = box Node::Cons(body.clone(), result.clone());
                first(&self.eval(&function))
            }
        };
        eprintln!("{:?} => {:?}", name, result);
        result
    }
}
