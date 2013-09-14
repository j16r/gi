use vm::parser::*;

struct Environment {
  world: @mut Token,
  nil: @mut Token,
  atom_true: @mut Token
}

macro_rules! define_internal_function(
  ($name:expr $function:ident) => (
    @mut Cons(@mut Atom(~$name), @mut Cons(@mut Function, @mut Nil));
  );
)

fn append(list: @mut Token, token: @mut Token) {
  match *list {
    Cons(_, ref mut tail @ @Nil) =>
      **tail = Cons(token, @mut Nil),
    Cons(_, ref mut rest) => append(*rest, token),
    _ => fail!("appending() on non list token")
  }
}

#[test]
fn test_append() {
  let mut list = @Cons(@Atom(~"first"), @mut Nil);

  append(list, @Atom(~"second"));
  dump_token(list);
  assert!(list ==
          @Cons(@Atom(~"first"), @Cons(@Atom(~"second"), @mut Nil)));
}

pub fn name(token: &Token) -> ~str {
  match *token {
    Atom(ref name) => name.clone(),
    _ => fail!("Applied name function to non Atom object"),
  }
}

fn eval_impl(token: &Token) -> @mut Token {
  @mut Nil
}

fn dump_token(token: &Token) -> ~str {
  match *token {
    Nil => ~"Nil",
    Function() => ~"Function",
    Lambda(_, _) => ~"Lambda()",
    Atom(ref text) => fmt!("Atom(%s)", *text),
    Cons(ref first, ref rest) =>
      fmt!("Cons(%s, %s)", dump_token(*first), dump_token(*rest))
  }
}

pub fn first(token: @mut Token) -> @mut Token {
  match *token {
    Cons(ref left, _) => *left,
    _ => @mut Nil
  }
}

pub fn rest(token: @mut Token) -> @mut Token {
  match *token {
    Cons(_, ref right) => *right,
    _ => @mut Nil
  }
}

fn quote_impl(env: &Environment, arguments: @mut Token) -> @mut Token {
  first(arguments)
}

fn first_impl(env: &Environment, arguments: @mut Token) -> @mut Token {
  first(arguments)
}

fn rest_impl(env: &Environment, arguments: @mut Token) -> @mut Token {
  rest(arguments)
}

fn cons_impl(env: &Environment, _: @mut Token) -> @mut Token {
  @mut Nil
}

fn equal_impl(env: &Environment, arguments: @mut Token) -> @mut Token {
  let left = first(arguments);
  let right = rest(arguments);
  if name(left) == name(right) {
    env.atom_true
  } else {
    env.nil
  }
}

fn atom_impl(env: &Environment, _: @mut Token) -> @mut Token {
  @mut Nil
}

fn cond_impl(env: &Environment, _: @mut Token) -> @mut Token {
  @mut Nil
}

fn lambda_impl(env: &Environment, _: @mut Token) -> @mut Token {
  @mut Nil
}

fn label_impl(env: &Environment, _: @mut Token) -> @mut Token {
  @mut Nil
}

impl Environment {
  pub fn new() -> ~Environment {
    let mut world =
      @mut Cons(define_internal_function!("quote" quote_impl), @mut Nil);
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
      nil: @mut Cons(@mut Nil, @mut Nil),
      atom_true: @mut Atom(~"#true")
    }
  }

  pub fn eval(&mut self, token: @mut Token) -> @mut Token {
    match *token {
      Cons(ref left, _) => {
        match *left {
          @Atom(~"lambda") => {
            let largs = first(rest(token));
            let lsexp = first(rest(rest(token)));
            @mut Lambda(largs, lsexp)
          },
          _ => {
            let mut accum = @mut Cons(self.eval(first(token)), @mut Nil);
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

  fn lookup(&self, token: ~str) -> @mut Token {
    lookup_tail(self.world, token)
  }
}

fn lookup_tail(cursor: &Token, token: ~str) -> @mut Token {
  match *cursor {
    Cons(ref item, ref tail) => {
      //println("Lookup tail Cons");
      if name(first(*item)) == token {
        first(*tail)
      } else {
        lookup_tail(*tail, token)
      }
    },
    _ => @mut Nil
  }
}
