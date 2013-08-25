struct Parser {
  current_expression: ~str
}

#[deriving(Clone)]
pub enum Token {
  Empty,
  Function,
  Lambda(~Token, ~Token),
  Atom(~str),
  Cons(~Token, ~Token)
}

impl Parser {
  pub fn new() -> ~Parser {
    ~Parser{current_expression: ~""}
  }

  pub fn parse(&mut self, line: &str) -> ~Token {
    self.current_expression = line.to_owned();
    match self.next_token() {
      ~Atom(~"(") => {
        self.parse_tail()
      },
      _ => ~Empty
    }
  }

  fn parse_tail(&mut self) -> ~Token {
    match self.next_token() {
      ~Atom(~")") => (),
      ~Atom(~"(") => {
        let left = self.parse_tail();
        let right = self.parse_tail();
        return ~Cons(left, right)
      },
      ~Atom(text) => {
        println(fmt!("Token: Atom = %s", text));
        let left = ~Atom(text);
        let right = self.parse_tail();
        return ~Cons(left, right)
      },
      token => {
        let left = token;
        let right = self.parse_tail();
        return ~Cons(left, right)
      }
    }
    ~Empty
  }

  fn next_token(&mut self) -> ~Token {
    let mut ch = self.current_expression.shift_char();

    while(ch.is_whitespace()) {
      ch = self.current_expression.shift_char();
    }

    if(ch == '\n') {
      self.current_expression.shift_char();
    }

    if(ch == ')') {
      return ~Atom(~")");
    }
    if(ch == '(') {
      return ~Atom(~"(");
    }

    let mut token = ~"";
    while(!ch.is_whitespace() && ch != ')') {
      token.push_char(ch);
      ch = self.current_expression.shift_char();
    }

    if(ch == ')') {
      self.current_expression.unshift_char(ch);
    }

    ~Atom(token)
  }
}
