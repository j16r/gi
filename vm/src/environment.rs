use vm::parser::*;

struct Environment {
  world: ~Token,
  nil: ~Token,
  atom_true: ~Token
}

macro_rules! define_internal_function(
  ($name:expr $function:expr) => (
    ~Cons(~Atom(~$name), ~Cons(~Function, ~Nil));
  );
)

fn append(list: &mut Token, token: ~Token) {
  match *list {
    Cons(_, ref mut tail @ ~Nil) => 
      **tail = Cons(token, ~Nil),
    Cons(_, ref mut rest) => append(*rest, token),
    _ => fail!("appending() on non list token")
  }
} 

#[test]
fn test_append() {
  let mut list = ~Cons(~Atom(~"first"), ~Nil);

  append(list, ~Atom(~"second"));
  dump_token(list);
  assert!(list == 
          ~Cons(~Atom(~"first"), ~Cons(~Atom(~"second"), ~Nil)));
} 

pub fn name(token: &Token) -> ~str {
  match token {
    &Atom(ref name) => name.clone(),
    _ => fail!("Applied name function to non Atom object"),
  }
}

fn eval_impl(token: &Token) -> ~Token {
  ~Nil
}

fn dump_token(token: &Token) -> ~str {
  match token {
    &Nil => ~"Nil",
    &Function => ~"Function",
    &Lambda(_, _) => ~"Lambda()",
    &Atom(ref text) => fmt!("Atom(%s)", *text),
    &Cons(ref first, ref rest) => 
      fmt!("Cons(%s, %s)", dump_token(*first), dump_token(*rest))
  }
}

pub fn first(token: &Token) -> ~Token {
  match *token {
    Cons(ref left, _) => left.clone(),
    _ => ~Nil
  }
}

pub fn rest(token: &Token) -> ~Token {
  match *token {
    Cons(_, ref right) => right.clone(),
    _ => ~Nil
  }
}

impl Environment {
  pub fn new() -> ~Environment {
    let mut world =
      ~Cons(define_internal_function!("quote" quote_impl), ~Nil);
    append(world, define_internal_function!("first" first_impl));
    append(world, define_internal_function!("rest" rest_impl));
    append(world, define_internal_function!("cons" cons_impl));
    append(world, define_internal_function!("equal" equal_impl));
    append(world, define_internal_function!("atom" atom_impl));
    append(world, define_internal_function!("cond" cond_impl));
    append(world, define_internal_function!("lambda" lambda_impl));
    append(world, define_internal_function!("label" label_impl));

    println("world------------------------------");
    println(dump_token(world));
    println("world------------------------------");

    ~Environment{
      world: world,
      nil: ~Cons(~Nil, ~Nil),
      atom_true: ~Atom(~"#true")
    }
  }

  fn quote_impl(&self, arguments: ~Token) -> ~Token {
    first(arguments)
  }

  fn first_impl(&self, arguments: ~Token) -> ~Token {
    first(arguments)
  }

  fn rest_impl(&self, arguments: ~Token) -> ~Token {
    rest(arguments)
  }

  fn cons_impl(&self, _: ~Token) -> ~Token {
    ~Nil
  }

  fn equal_impl(&self, arguments: ~Token) -> ~Token {
    let left = first(arguments);
    let right = rest(arguments);
    if name(left) == name(right) {
      self.atom_true.clone()
    } else {
      self.nil.clone()
    }
  }

  fn atom_impl(&self, _: ~Token) -> ~Token {
    ~Nil
  }

  fn cond_impl(&self, _: ~Token) -> ~Token {
    ~Nil
  }

  fn lambda_impl(&self, _: ~Token) -> ~Token {
    ~Nil
  }

  fn label_impl(&self, _: ~Token) -> ~Token {
    ~Nil
  }

  pub fn eval(&mut self, token: &Token) -> ~Token {
    match *token {
      Cons(ref left, _) => {
        match *left {
          ~Atom(~"lambda") => {
            let largs = first(rest(token));
            let lsexp = first(rest(rest(token)));
            ~Lambda(largs, lsexp)
          },
          _ => {
            let mut accum = ~Cons(self.eval(first(token)), ~Nil);
            let mut sexp = rest(token);

            loop {
              match *sexp {
                Cons(_, _) => {
                  append(accum, self.eval(first(sexp)));
                  sexp = rest(sexp);
                },
                _ => break,
              }
            }

            eval_impl(accum)
          },
        }
      },
      _ => self.lookup(name(token)),
    }
  }

  fn lookup(&self, token: ~str) -> ~Token {
    lookup_tail(self.world, token)
  }
}

fn lookup_tail(cursor: &Token, token: ~str) -> ~Token {
  match *cursor {
    Cons(ref item, ref tail) => {
      //println("Lookup tail Cons");
      if name(first(*item)) == token {
        first(*tail)
      } else {
        lookup_tail(*tail, token)
      }
    },
    _ => ~Nil
  }
}
