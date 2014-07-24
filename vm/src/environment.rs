use std::mem::swap;
use std::str::{MaybeOwned, Slice, Owned};

static ATOM_NIL : Token = Nil;
//static ATOM_TRUE : Token = Atom("#true".to_string());
//static ATOM_FALSE : Token = Atom("#false".to_string());

pub struct Environment {
  world: Box<Token>,
}

pub enum Token {
  Nil,
  True,
  False,
  OpenParen,
  CloseParen,
  Atom(String),
  Function(extern "Rust" fn(&Environment, &Token) -> Box<Token>),
  Lambda(Box<Token>, Box<Token>),
  Cons(Box<Token>, Box<Token>)
}

macro_rules! define_internal_function(
  ($name:expr $function:ident) => (
    box Cons(box Atom(box $name),
             box Cons(box Function($function), box Nil));
  );
)

fn append(list: &mut Box<Token>, token: Box<Token>) {
  match *list {
    box Cons(_, ref mut tail @ box Nil) => {
      swap(tail, &mut box Cons(token, box Nil))
    }
    box Cons(_, ref mut rest) => {
      append(rest, token)
    },
    _ => fail!("appending() on non list token")
  }
}

#[test]
fn test_append() {
  let mut list = box Cons(box Atom(box "first"), box Nil);

  append(list, box Atom(box "second"));
  dump_token(list);
  assert!(list ==
          box Cons(box Atom(box "first"),
                    box Cons(box Atom(box "second"), box Nil)));
}

pub fn name(token: &Box<Token>) -> String {
  match *token {
    box Atom(ref name) => name.clone(),
    _ => fail!(format!("Applied name function to non Atom object {}", dump_token(token))),
  }
}

fn eval_impl<'r>(env: &Environment, token: &'r Box<Token>) -> &'r Box<Token> {
  println!("eval_impl");
  let symbol = first(token);
  //let args = rest(token);

  match *symbol {
    //Lambda(_, _) => *env.atom_nil,
    //Function(function) => function(env, token),
    _ => token,
  }
}

pub fn dump_token<'a>(token: &Box<Token>) -> MaybeOwned<'a> {
  match *token {
    box Nil =>
      Slice("Nil"),
    box Function(_) =>
      Slice("Function"),
    box Lambda(_, _) =>
      Slice("Lambda()"),
    box Atom(ref text) =>
      Owned(format!("Atom({})", *text)),
    box OpenParen =>
      Slice("("),
    box CloseParen =>
      Slice(")"),
    box Cons(ref first, ref rest) =>
      Owned(format!("Cons({}, {})", dump_token(first), dump_token(rest))),
    _ => fail!("U R LAZY!")
  }
}

pub fn first(token: &Box<Token>) -> &Box<Token> {
  match *token {
    box Cons(ref first, _) => first,
    _ => &box ATOM_NIL
  }
}

pub fn rest(token: &Box<Token>) -> &Box<Token> {
  match *token {
    box Cons(_, ref rest) => rest,
    _ => &box ATOM_NIL
  }
}

pub fn next(token: &Box<Token>) -> &Box<Token> {
  first(rest(token))
}

//fn quote_impl(env: &Environment, arguments: Box<Token>) -> Box<Token> {
  //println("quote_impl");
  //first(arguments)
//}

//fn first_impl(env: &Environment, arguments: Box<Token>) -> Box<Token> {
  //println("first_impl");
  //first(first(arguments))
//}

//fn rest_impl(env: &Environment, arguments: Box<Token>) -> Box<Token> {
  //println("rest_impl");
  //rest(first(arguments))
//}

//fn cons_impl(env: &Environment, arguments: Box<Token>) -> Box<Token> {
  //println("cons_impl");
  //let list = Box<Cons>(first(arguments), Box<Nil>);
  //let mut args = first(rest(arguments));

  //loop {
    //match *args {
      //Cons(_, _) => {
        //append(list, first(args));
        //args = rest(args);
      //},
      //_ => break
    //}
  //}

  //list

  ////object *list = cons(car(args),NULL);
  ////args = car(cdr(args));

  ////while (args != NULL && args->type == CONS){
    ////append(list,car(args));
    ////args = cdr(args);
  ////}

  ////return list;
//}

//fn equal_impl(env: &Environment, arguments: Box<Token>) -> Box<Token> {
  //println("equal_impl");
  //let left = first(arguments);
  //let right = rest(arguments);
  //if name(left) == name(right) {
    //env.atom_true
  //} else {
    //env.nil
  //}
//}

//fn atom_impl(env: &Environment, _: Box<Token>) -> Box<Token> {
  //println("atom_impl");
  //Box<Nil>
//}

//fn cond_impl(env: &Environment, _: Box<Token>) -> Box<Token> {
  //println("cond_impl");
  //Box<Nil>
//}

//fn lambda_impl(env: &Environment, _: Box<Token>) -> Box<Token> {
  //println("lambda_impl");
  //Box<Nil>
//}

//fn label_impl(env: &Environment, _: Box<Token>) -> Box<Token> {
  //println("label_impl");
  //Box<Nil>
//}

impl Environment {
  pub fn new() -> Box<Environment> {
    //let mut world =
      //Box<Cons>(define_internal_function!("quote" quote_impl), Box<Nil>);
    //append(world, define_internal_function!("first" first_impl));
    //append(world, define_internal_function!("rest" rest_impl));
    //append(world, define_internal_function!("cons" cons_impl));
    //append(world, define_internal_function!("equal" equal_impl));
    //append(world, define_internal_function!("atom" atom_impl));
    //append(world, define_internal_function!("cond" cond_impl));
    //append(world, define_internal_function!("lambda" lambda_impl));
    //append(world, define_internal_function!("label" label_impl));

    //println("world------------------------------");
    //println(dump_token(world));
    //println("world------------------------------");

    box Environment {
      world: box Cons(box Nil, box Nil),
    }
  }

  pub fn eval(&mut self, token: &Box<Token>) -> Box<Token> {
    match *token {
      box Cons(left, _) => {
        match left {
          box Atom(ref desc) if *desc == "lambda".to_string() => {
            let largs = first(rest(token));
            let lsexp = first(rest(rest(token)));
            box Lambda(*largs, *lsexp)
          },
          _ => fail!("Uh oh!")
            //let mut accum = box Cons(self.eval(box *first(token)), box Nil);
            //let mut sexp = rest(token);

            //loop {
              //match *sexp {
                //Cons(_, _) => {
                  //append(accum, self.eval(box *first(sexp)));
                  //sexp = rest(sexp);
                //},
                //_ => break,
              //}
            //}

            //eval_impl(self, accum)
          //},
        }
      },
      _ => self.lookup(name(token)).clone(),
    }
  }

  fn lookup(&self, token: String) -> &Box<Token> {
    lookup_tail(&self.world, token)
  }
}

fn lookup_tail(cursor: &Box<Token>, token: String) -> &Box<Token> {
  match *cursor {
    box Cons(ref item, ref tail) => {
      if name(first(item)) == token {
        first(tail)
      } else {
        lookup_tail(tail, token)
      }
    },
    _ => &box ATOM_NIL
  }
}

#[test]
fn test_lookup_tail() {
  let list = box Cons(
    box Cons(box Nil, box Nil),
    box Cons(
      box Cons(
        box Cons(box Atom("needle".to_string()), box Atom("prize".to_string())),
        box Cons(box Cons(box Atom("haystack".to_string()), box Nil), box Nil))));

  assert!(lookup_tail(list, "needle".to_string()) == box Atom("prize".to_string()))
}
