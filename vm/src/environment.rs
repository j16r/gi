use vm::parser::*;

struct Environment {
  world: ~Token,
  nil: ~Token,
  atom_true: ~Token
}

macro_rules! define_internal_function(
  ($name:expr $function:expr) => (
    ~Cons(~Atom(~$name), ~Cons(~Function, ~Empty));
  );
)

fn append(list: &mut Token, token: &Token) {
  let mut ptr = list;
  //loop {
    match ptr {
      //&Empty => break,
      &Cons(_, ref mut rest) => {
        *rest = ~Cons(~token.clone(), ~Empty);
        //ptr = rest;
      },
      _ => fail!("Tried to append to non-list object"),
    }
  //}
}

pub fn name(token: &Token) -> ~str {
  match token {
    &Atom(ref name) => name.clone(),
    _ => fail!("Applied name function to non Atom object"),
  }
}

fn eval_impl(token: &Token) -> ~Token {
  dump_token(token);
  ~Empty
}

fn dump_token(token: &Token) {
  match token {
    &Empty => println("Empty"),
    &Function => println("Function"),
    &Lambda(_, _) => println("Lambda()"),
    &Atom(ref text) => println(fmt!("Atom(%s)", *text)),
    &Cons(_, _) => println("Cons()")
  }
}

pub fn first(token: &Token) -> ~Token {
  match token {
    &Cons(ref left, _) => left.clone(),
    _ => ~Empty
  }
}

pub fn rest(token: &Token) -> ~Token {
  match token {
    &Cons(_, ref right) => right.clone(),
    _ => ~Empty
  }
}

impl Environment {
  pub fn new() -> ~Environment {
    let mut world =
      ~Cons(define_internal_function!("quote" quote_impl), ~Empty);
    append(world, define_internal_function!("first" first_impl));
    append(world, define_internal_function!("rest" rest_impl));
    append(world, define_internal_function!("cons" cons_impl));
    append(world, define_internal_function!("equal" equal_impl));
    append(world, define_internal_function!("atom" atom_impl));
    append(world, define_internal_function!("cond" cond_impl));
    append(world, define_internal_function!("lambda" lambda_impl));
    append(world, define_internal_function!("label" label_impl));

    ~Environment{
      world: world,
      nil: ~Cons(~Empty, ~Empty),
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

  fn cons_impl(&self, arguments: ~Token) -> ~Token {
    ~Empty
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

  fn atom_impl(&self, arguments: ~Token) -> ~Token {
    ~Empty
  }

  fn cond_impl(&self, arguments: ~Token) -> ~Token {
    ~Empty
  }

  fn lambda_impl(&self, arguments: ~Token) -> ~Token {
    ~Empty
  }

  fn label_impl(&self, arguments: ~Token) -> ~Token {
    ~Empty
  }

  pub fn eval(&mut self, token: &Token) -> ~Token {
    match token {
      &Cons(ref left, ref right) => {
        match left {
          &~Atom(~"lambda") => {
            let largs = first(rest(token));
            let lsexp = first(rest(rest(token)));
            ~Lambda(largs, lsexp)
          },
          _ => {
            let mut accum = ~Cons(self.eval(first(token)), ~Empty);
            let mut sexp = rest(token);

            loop {
              match sexp {
                ~Cons(_, _) => {
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
    let mut ptr = self.world;
    loop {
      match ptr {
        ~Cons(_, _) => {
          let item = first(ptr);
          let nm = first(item);
          let val = first(rest(item));

          if name(nm) == token {
            return val;
          }

          ptr = rest(ptr);
        },
        _ => break,
      }
    }
    ~Empty
  }
}
