use vm::parser::*;

pub fn create_environment() -> ~Token {
  let environment =
      ~Cons(~Cons(~Atom(~"quote"), ~Cons(~Function, ~Empty)), ~Empty);

  environment

  //object *env = cons(cons(atom("QUOTE"),cons(func(&fn_quote),NULL)),NULL);
  //append(env,cons(atom("CAR"),cons(func(&fn_car),NULL)));
  //append(env,cons(atom("CDR"),cons(func(&fn_cdr),NULL)));
  //append(env,cons(atom("CONS"),cons(func(&fn_cons),NULL)));
  //append(env,cons(atom("EQUAL"),cons(func(&fn_equal),NULL)));
  //append(env,cons(atom("ATOM"),cons(func(&fn_atom),NULL)));
  //append(env,cons(atom("COND"),cons(func(&fn_cond),NULL)));
  //append(env,cons(atom("LAMBDA"),cons(func(&fn_lambda),NULL)));
  //append(env,cons(atom("LABEL"),cons(func(&fn_label),NULL)));
}
