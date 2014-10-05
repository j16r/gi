use ast::{Node, Nil, Atom, Cons};
use grammar::{Token, OpenParen, CloseParen, Identifier};
use std::fmt;

struct Parser {
  input: String,
  position: uint,
  length: uint
}

pub struct ParseError {
  line_number: uint,
  character: uint,
  explanation: &'static str
}

impl fmt::Show for ParseError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}:{}", self.line_number, self.character)
  }
}

type ParserResult = Result<Box<Node>, ParseError>;
type LexerResult = Result<Box<Token>, ParseError>;

pub fn parse(program: String) -> ParserResult {
  let mut parser = Parser {
    input: program.to_string(),
    position: 0,
    length: program.len()
  };

  println!("Parsing {}", program);

  match *try!(parser.next_token()) {
    OpenParen => {
      parser.parse_tail()
    },
    _ => Ok(box Nil)
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
      Identifier(name) => {
        let left = box Atom(name);
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

  fn next_token(&mut self) -> LexerResult {
    if self.eof() {
      return Err(ParseError {
        line_number: 0,
        character: self.position,
        explanation: "End of file?"});
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
      token.push(ch);
      ch = self.current_char();
      self.advance_char();
    }

    if ch == ')' {
      self.rewind_char();
    }

    Ok(box Identifier(token))
  }
}

fn assert_parse_tree(input : &str, output : &str) {
  let ast = parse(input.to_string());
  let formatted = ast.unwrap().to_string();
  assert_eq!(formatted, output.to_string());
}

#[test]
fn test_parse_empty_program() {
  assert!(parse("".to_string()).is_err());
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
