use std::collections::HashMap;
use ast::Node;
use ast::Node::{Nil, Atom, Cons, Integer32, Bool};

type Builtin = fn (&mut Environment, &Box<Node>) -> Box<Node>;

pub struct Environment {
    functions: Box<HashMap<String, Box<Node>>>
}

fn first(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, _) => box Cons(lhs_token.clone(), box Nil),
        _ => panic!("WTF?")
    }
}

fn rest(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(_, ref rhs_token) => box Cons(rhs_token.clone(), box Nil),
        _ => panic!("WTF?")
    }
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

fn add(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, ref tail) => {
            match *lhs_token {
                box Integer32(ref lhs_value) => {
                    match *tail {
                        box Cons(ref rhs_token, _) => {
                            match *rhs_token {
                                box Integer32(ref rhs_value) => {
                                    box Integer32(*lhs_value + *rhs_value)
                                },
                                _ => panic!("second argument to add must be an Atom")
                            }
                        },
                        _ => panic!("add only takes two arguments, got more")
                    }
                },
                _ => panic!("first argument to add must be an Atom")
            }
        },
        _ => panic!("add requires two arguments")
    }
}

fn mul(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, ref tail) => {
            match *lhs_token {
                box Integer32(ref lhs_value) => {
                    match *tail {
                        box Cons(ref rhs_token, _) => {
                            match *rhs_token {
                                box Integer32(ref rhs_value) => {
                                    box Integer32(*lhs_value * *rhs_value)
                                },
                                _ => panic!("second argument to mul must be an Atom")
                            }
                        },
                        _ => panic!("mul only takes two arguments, got more")
                    }
                },
                _ => panic!("first argument to mul must be an Atom")
            }
        },
        _ => panic!("mul requires two arguments")
    }
}

fn cond(env: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, ref tail) => {
            match *lhs_token {
                box Bool(expression) => {
                    if expression {
                        return env.eval(tail);
                    }
                },
                _ => panic!("first argument to cond must be an Bool")
            }
        },
        _ => panic!("cond requires at least two arguments")
    }

    box Node::Nil
}

fn div(_: &mut Environment, args: &Box<Node>) -> Box<Node> {
    match *args {
        box Cons(ref lhs_token, ref tail) => {
            match *lhs_token {
                box Integer32(ref lhs_value) => {
                    match *tail {
                        box Cons(ref rhs_token, _) => {
                            match *rhs_token {
                                box Integer32(ref rhs_value) => {
                                    box Integer32(*lhs_value / *rhs_value)
                                },
                                _ => panic!("second argument to div must be an Atom")
                            }
                        },
                        _ => panic!("div only takes two arguments, got more")
                    }
                },
                _ => panic!("first argument to div must be an Atom")
            }
        },
        _ => panic!("div requires two arguments")
    }
}

impl Environment {
    pub fn new() -> Box<Environment> {
        box Environment {
            functions: box HashMap::new()
        }
    }

    pub fn eval(&mut self, token: &Box<Node>) -> Box<Node> {
        match *token {
            box Cons(ref head, ref tail) => {
                let result = &self.eval(tail);
                match *head {
                    box Atom(ref value) => self.invoke_function(value, result),
                    box Cons(_, _) => self.eval(head),
                    _ => token.clone()
                }
            },
            _ => {
                token.clone()
            }
        }
    }

    fn invoke_function(&mut self, name: &String, args: &Box<Node>) -> Box<Node> {
        match &name[..] {
            "first" => {
                let result = &first(self, args);
                self.eval(result)
            },
            "rest" => {
                let result = &rest(self, args);
                self.eval(result)
            },
            "println" => {
                let result = &println(self, args);
                self.eval(result)
            },
            "dumpln" => {
                let result = &dumpln(self, args);
                self.eval(result)
            },
            "dump" => {
                let result = &dump(self, args);
                self.eval(result)
            },
            "add" => {
                let result = &add(self, args);
                self.eval(result)
            },
            "mul" => {
                let result = &mul(self, args);
                self.eval(result)
            },
            "div" => {
                let result = &div(self, args);
                self.eval(result)
            },
            "cond" => {
                let result = &cond(self, args);
                self.eval(result)
            },
            _ => {
                match self.functions.get(name).cloned() {
                    Some(function) => self.eval(&function),
                    None => panic!("Tried to invoke function {:?} but there was none in scope", name)
                }
            }
        }
    }
}
