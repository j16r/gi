use std;

fn twice(x: int) -> int { x + x }

#[test]
fn test_twice() {
  let mut i = -100;
  while i < 100 {
    assert twice(i) == 2 * i;
    i += 1;
  }
}

