
pub enum Token {
  LeafToken,
  BranchToken
}

pub enum LeafToken {
  Nil,
  True,
  False,
  Atom(String)
}

pub enum BranchToken {
  Cons(Box<Token>, Box<Token>),
  LCons(Box<Token>, LeafToken),
  RCons(LeafToken, Box<Token>),
  BCons(LeafToken, LeafToken)
}

#[test]
fn test_ast() {
  let tree : Token = BranchToken(box BCons(True, Nil));
}
