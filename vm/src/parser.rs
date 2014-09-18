use ast::{Token, Nil, Atom, OpenParen, CloseParen, Cons};
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

pub fn dump(ast : &Box<Token>) -> String {
  match *ast {
    box Nil => "Nil".to_string(),
    box OpenParen => "(".to_string(),
    box CloseParen => ")".to_string(),
    box Atom(ref token) => token.to_string(),
    box Cons(ref first, ref rest) => {
      "Cons(".to_string() + dump(first) + ", " + dump(rest) + ")"
    },
    _ => "".to_string()
  }
}

impl Parser {
  fn parse_tail(&mut self) -> ParserResult {
    let token = try!(self.next_token());
    match *token {
      OpenParen => {
        let left = try!(self.parse_tail());
        let right = try!(self.parse_tail());
        Ok(box Cons(left, right))
      },
      CloseParen => {
        Ok(box Nil)
      },
      Atom(ref text) => {
        let left = box Atom(text.to_string());
        let right = try!(self.parse_tail());
        Ok(box Cons(left, right))
      },
      _ => {
        let left = token;
        let right = try!(self.parse_tail());
        Ok(box Cons(left, right))
      }
    }
  }

  fn eof(&self) -> bool {
    self.position == self.length
  }

  fn current_char(&self) -> char {
    self.input.as_slice().char_at(self.position)
  }

  fn advance_char(&mut self) {
    self.position += 1;
  }

  fn rewind_char(&mut self) {
    self.position -= 1;
  }

  fn next_token(&mut self) -> ParserResult {
    if self.eof() {
      return Ok(box Nil);
    }

    let mut ch = self.current_char();
    self.advance_char();

    while ch.is_whitespace() {
      self.advance_char();
    }

    if ch == ')' {
      return Ok(box CloseParen);
    }
    if ch == '(' {
      return Ok(box OpenParen);
    }

    let mut token = String::new();
    while !ch.is_whitespace() && ch != ')' {
      token.push_char(ch);
      ch = self.current_char();
      self.advance_char();
    }

    if ch == ')' {
      self.rewind_char();
    }

    Ok(box Atom(token))
  }
}

fn assert_parse_tree(input : &str, output : &str) {
  let ast = parse(input.to_string());
  let formatted = dump(&ast.unwrap());
  assert_eq!(formatted, output.to_string());
}

#[test]
fn test_parse_empty_program() {
  assert_parse_tree("", "Nil");
}

#[test]
fn test_parse_empty_list() {
  assert_parse_tree("()", "Nil");
}

#[test]
fn test_parse_function() {
  assert_parse_tree("(abort)", "Cons(abort, Nil)");
}

#[test]
fn test_parse_nested_function() {
  assert_parse_tree(
    "(println (conj 1 2))",
    "Cons(println, Cons(Cons(conj, Cons(1, Cons(2, Nil))), Nil))");
}
