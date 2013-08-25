use vm::parser::*;
use vm::eval::*;

macro_rules! define_internal_function(
  ($name:expr $function:expr) => (
    ~Cons(~Atom(~$name), ~Cons(~Function, ~Empty));
  );
)

pub fn create_environment() -> ~Token {
  let mut environment =
    ~Cons(define_internal_function!("quote" quote_impl), ~Empty);
  append(environment, define_internal_function!("first" first_impl));
  append(environment, define_internal_function!("rest" rest_impl));
  append(environment, define_internal_function!("cons" cons_impl));
  append(environment, define_internal_function!("equal" equal_impl));
  append(environment, define_internal_function!("atom" atom_impl));
  append(environment, define_internal_function!("cond" cond_impl));
  append(environment, define_internal_function!("lambda" lambda_impl));
  append(environment, define_internal_function!("label" label_impl));
  environment
}

fn quote_impl(arguments: ~Token, env: ~Token) -> ~Token {
  first(arguments)
}

fn first_impl(arguments: ~Token, env: ~Token) -> ~Token {
  first(arguments)
}

fn rest_impl(arguments: ~Token, env: ~Token) -> ~Token {
  rest(arguments)
}

fn cons_impl(arguments: ~Token, env: ~Token) -> ~Token {
  ~Empty
}

fn equal_impl(arguments: ~Token, env: ~Token) -> ~Token {
  ~Empty
}

fn atom_impl(arguments: ~Token, env: ~Token) -> ~Token {
  ~Empty
}

fn cond_impl(arguments: ~Token, env: ~Token) -> ~Token {
  ~Empty
}

fn lambda_impl(arguments: ~Token, env: ~Token) -> ~Token {
  ~Empty
}

fn label_impl(arguments: ~Token, env: ~Token) -> ~Token {
  ~Empty
}
