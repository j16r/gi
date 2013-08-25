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
  //println(fmt!("After skip %s", fixed_text));
  assert!(it.to_owned_vec() == ~['t', 'r', 'i', 'm']);
}

fn slurp_token(it : &mut CharIterator) -> ~str {
  it.take_while(|ch| !ch.is_whitespace()).collect()
}

#[test]
fn test_slurp_token() {
  let text = ~"token TOKEN";
  let mut it = text.iter();

  println(fmt!("Token: %s", slurp_token(&mut it)));
  //assert!(slurp_token(&mut it) == ~"token");
  println(fmt!("Token: %s", slurp_token(&mut it)));
  //assert!(slurp_token(&mut it) == ~"TOKEN");
}
