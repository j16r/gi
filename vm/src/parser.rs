use vm::environment::*;

struct Parser {
  current_expression: ~str
}

impl Parser {
  pub fn new() -> ~Parser {
    ~Parser{current_expression: ~""}
  }

  pub fn parse(&mut self, line: &str) -> @mut Token {
    self.current_expression = line.to_owned();
    println(fmt!("Parsing line %s", self.current_expression));
    match *self.next_token() {
      Atom(~"(") => {
        println("(");
        self.parse_tail()
      },
      _ => @mut Nil
    }
  }

  fn parse_tail(&mut self) -> @mut Token {
    let token = self.next_token();
    match *token {
      Atom(~")") => {
        println(")");
        @mut Nil
      },
      Atom(~"(") => {
        println("(");
        let left = self.parse_tail();
        let right = self.parse_tail();
        @mut Cons(left, right)
      },
      Atom(ref text) => {
        println(fmt!("Token: Atom = %s", *text));
        let left = @mut Atom(text.clone());
        let right = self.parse_tail();
        @mut Cons(left, right)
      },
      _ => {
        println("token");
        let left = token;
        let right = self.parse_tail();
        @mut Cons(left, right)
      }
    }
  }

  fn next_token(&mut self) -> @mut Token {
    let mut ch = self.current_expression.shift_char();

    while(ch.is_whitespace()) {
      ch = self.current_expression.shift_char();
    }

    if(ch == '\n') {
      self.current_expression.shift_char();
    }

    if(ch == ')') {
      return @mut Atom(~")");
    }
    if(ch == '(') {
      return @mut Atom(~"(");
    }

    let mut token = ~"";
    while(!ch.is_whitespace() && ch != ')') {
      token.push_char(ch);
      ch = self.current_expression.shift_char();
    }

    if(ch == ')') {
      self.current_expression.unshift_char(ch);
    }

    @mut Atom(token)
  }
}
