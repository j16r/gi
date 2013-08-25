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

  fn lookup(&self, token: &str) -> ~Token {
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

}
