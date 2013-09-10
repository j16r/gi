struct Parser {
  current_expression: ~str
}

#[deriving(Clone, Eq)]
pub enum Token {
  Nil,
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
    println(fmt!("Parsing line %s", self.current_expression));
    match *self.next_token() {
      Atom(~"(") => {
        println("(");
        self.parse_tail()
      },
      _ => ~Nil
    }
  }

  fn parse_tail(&mut self) -> ~Token {
    match self.next_token() {
      ~Atom(~")") => {
        println(")");
        ~Nil
      },
      ~Atom(~"(") => {
        println("(");
        let left = self.parse_tail();
        let right = self.parse_tail();
        ~Cons(left, right)
      },
      ~Atom(ref text) => {
        println(fmt!("Token: Atom = %s", *text));
        let left = ~Atom(text.clone());
        let right = self.parse_tail();
        ~Cons(left, right)
      },
      token => {
        println("token");
        let left = token;
        let right = self.parse_tail();
        ~Cons(left, right)
      }
    }
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
