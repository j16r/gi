use std::io;
use std::str::CharIterator;

fn twice(x: int) -> int { x + x }

#[test]
fn test_twice() {
  let mut i = -100;
  while i < 100 {
    assert!(twice(i) == 2 * i);
    i += 1;
  }
}

fn multiple_returns() -> (int, char) {
  (0, 'a')
}

#[test]
fn test_multiple_returns() {
  let (x, y) = multiple_returns();
  assert!((x, y) == (0, 'a'));
}

fn skip(it : &mut CharIterator) {
  it.skip_while(|ch| ch.is_whitespace());
}

#[test]
fn test_skip() {
  let text = ~"  trim";
  let mut it = text.iter();

  skip(&mut it);

  //let fixed_text = it.to_owned();
  //println(format!("After skip {}", fixed_text));
  assert!(it.to_owned_vec() == ~['t', 'r', 'i', 'm']);
}

fn slurp_token(it : &mut CharIterator) -> ~str {
  it.take_while(|ch| !ch.is_whitespace()).collect()
}

#[test]
fn test_slurp_token() {
  let text = ~"token TOKEN";
  let mut it = text.iter();

  println(format!("Token: {}", slurp_token(&mut it)));
  //assert!(slurp_token(&mut it) == ~"token");
  println(format!("Token: {}", slurp_token(&mut it)));
  //assert!(slurp_token(&mut it) == ~"TOKEN");
}

#[deriving(Clone, Eq)]
struct List {
  next: Option<~List>,
  text: ~str
}

impl List {
  pub fn new() -> ~List {
    ~List{next: None, text: ~"head"}
  }

  pub fn append(&mut self, text: ~str) {
    match self.next {
      Some(ref mut next) => next.append(text),
      None => self.next = Some(~List{next: None, text: text}),
    }
  }
}

#[test]
fn test_list() {
  let mut list = List::new();
  list.append(~"tail");
  assert!(list == ~List{next: Some(~List{next: None, text: ~"tail"}), text: ~"head"});
}

enum FuncHolder {
  Func(fn()),
  Nil,
}

pub fn do_something() {
}

#[test]
fn test_func_holder() {
  let fn_tuple = Func(do_something);
}
