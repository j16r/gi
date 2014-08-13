use vm::ast::{Token, Nil, Atom, OpenParen, CloseParen, Cons};
use std::fmt;

struct Parser {
  input: String,
  position: uint,
  length: uint
}

pub struct ParseError {
  line_number: uint,
  character: uint
}

impl fmt::Show for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.line_number, self.character)
  }
}

type ParserResult = Result<Box<Token>, ParseError>;

pub fn parse(program: String) -> ParserResult {
  let mut parser = Parser {
    input: program.to_string(),
    position: 0,
    length: program.len()
  };

  println!("Parsing {}", program);

  match *try!(parser.next_token()) {
    OpenParen => {
      println!("(");
      parser.parse_tail()
    },
    _ => Ok(box Nil)
  }
}

pub fn dump(ast : Box<Token>) -> String {
  match *ast {
    OpenParen => "(".to_string(),
    CloseParen => ")".to_string(),
    _ => "".to_string()
  }
}

impl Parser {
  fn parse_tail(&mut self) -> ParserResult {
    let token = try!(self.next_token());
    match *token {
      CloseParen => {
        println!(")");
        Ok(box Nil)
      },
      OpenParen => {
        println!("(");
        let left = try!(self.parse_tail());
        let right = try!(self.parse_tail());
        Ok(box Cons(left, right))
      },
      Atom(ref text) => {
        println!("Token: Atom = {:s}", text.as_slice());
        let left = box Atom(text.to_string());
        let right = try!(self.parse_tail());
        Ok(box Cons(left, right))
      },
      _ => {
        println!("token");
        let left = token;
        let right = try!(self.parse_tail());
        Ok(box Cons(left, right))
      }
    }
  }

  fn current_char(&self) -> char {
    self.input.as_slice().char_at(self.position)
  }

  fn consume_char(&mut self) {
    self.position += 1;
  }

  fn next_token(&mut self) -> ParserResult {
    let mut ch = self.current_char();
    self.consume_char();

    while(ch.is_whitespace() || ch == '\n') {
      self.consume_char();
      ch = self.current_char();
    }

    if(ch == ')') {
      return Ok(box CloseParen);
    }
    if(ch == '(') {
      return Ok(box OpenParen);
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

    Ok(box Atom(token))
  }
}

fn assert_parse_tree(input : &str, output : &str) {
  let ast = parse(input.to_string());
  let formatted = dump(ast.unwrap());
  assert_eq!(formatted, output.to_string());
}

#[test]
fn test_parse_empty_program() {
  assert_parse_tree("", "");
}

#[test]
fn test_parse_empty_list() {
  assert_parse_tree("()", "()");
}
