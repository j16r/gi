use vm::environment::{Token, Nil, Function, Lambda, Atom, OpenParen, CloseParen, Cons};

struct Parser {
  input: String,
  position: uint,
  length: uint
}

pub fn parse(program: String) -> Box<Token> {
  let mut parser = Parser {
    input: program.to_string(),
    position: 0,
    length: program.len()
  };

  println!("Parsing {}", program);

  match *parser.next_token() {
    OpenParen => {
      println!("(");
      parser.parse_tail()
    },
    _ => box Nil
  }
}

impl Parser {

  fn parse_tail(&mut self) -> Box<Token> {
    let token = self.next_token();
    match *token {
      CloseParen => {
        println!(")");
        box Nil
      },
      OpenParen => {
        println!("(");
        let left = self.parse_tail();
        let right = self.parse_tail();
        box Cons(left, right)
      },
      Atom(ref text) => {
        println!("Token: Atom = {:s}", text.as_slice());
        let left = box Atom(text.to_string());
        let right = self.parse_tail();
        box Cons(left, right)
      },
      _ => {
        println!("token");
        let left = token;
        let right = self.parse_tail();
        box Cons(left, right)
      }
    }
  }

  fn current_char(&self) -> char {
    self.input.as_slice().char_at(self.position)
  }

  fn consume_char(&mut self) {
    self.position += 1;
  }

  fn next_token(&mut self) -> Box<Token> {
    let mut ch = self.current_char();
    self.consume_char();

    while(ch.is_whitespace() || ch == '\n') {
      self.consume_char();
      ch = self.current_char();
    }

    if(ch == ')') {
      return box CloseParen;
    }
    if(ch == '(') {
      return box OpenParen;
    }

    let mut token = "".to_string();
    while(!ch.is_whitespace() && ch != ')') {
      self.consume_char();
      token.push_char(ch);
      ch = self.current_char();
    }

    //if(ch == ')') { self.consume_char(ch);
      //self.current_expression.unshift_char(ch);
    //}

    box Atom(token)
  }
}

#[test]
fn test_parser() {
}
