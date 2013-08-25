use vm::parser::*;

pub fn eval(token: &Token, env: &Token) -> ~Token {
  match token {
    &Cons(ref left, ref right) => {
      match left {
        &~Atom(~"lambda") => {
          let largs = first(rest(token));
          let lsexp = first(rest(rest(token)));
          ~Lambda(largs, lsexp)
        },
        _ => {
          let mut accum = ~Cons(eval(first(token), env), ~Empty);
          let mut sexp = rest(token);

          loop {
            match sexp {
              ~Cons(_, _) => {
                append(accum, eval(first(sexp), env));
                sexp = rest(sexp);
              },
              _ => break,
            }
          }

          eval_impl(accum)
        },
      }
    },
    _ => lookup(name(token), env),
  }
}

pub fn append(list: &mut Token, token: &Token) {
  //let mut ptr = token;
  //loop {
    //match ptr {
      //~Empty => break,
      //_ => ()
    //}
    
    //ptr = rest(ptr);
  //}
}

//void append (object *list, object *obj) {
  //object *ptr;
  //for (ptr = list; cdr(ptr) != NULL; ptr = cdr(ptr));
  //cdr(ptr) = cons(obj, NULL);
//}

fn lookup(token: &str, env: &Token) -> ~Token {
  println(fmt!("Looking up %s", token));
  match token {
    "print" => println("Found print function!"),
    _ => ()
  }
  //object *tmp = env;

  //while (tmp != NULL && tmp->type == CONS) {
    //object *item = car(tmp);
    //object *nm = car(item);
    //object *val = car(cdr(item));

    //if(strcmp(name(nm),n) == 0)
      //return val;
    //tmp = cdr(tmp);
  //}
  //return NULL;
  ~Empty
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
