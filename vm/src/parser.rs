use std::fmt;
use std::io::Read;

#[cfg(test)]
use std::io::Cursor;

use ast::Node;
use grammar::Token;
use lexer::{Lexer, LexerError};

pub struct Parser<R> {
  lexer: Lexer<R>
}

pub struct ParserError {
  line_number: usize,
  column: usize,
  explanation: String
}

impl From<LexerError> for ParserError {
    fn from(error: LexerError) -> Self {
        ParserError {
            line_number: error.line_number,
            column: error.column,
            explanation: error.explanation
        }
    }
}

impl fmt::Debug for ParserError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}:{} {}",
           self.line_number,
           self.column,
           self.explanation)
  }
}

type ParserResult = Result<Box<Node>, ParserError>;

impl<R: Read> Parser<R> {
  pub fn new(reader: R) -> Parser<R> {
    Parser {
      lexer: Lexer::new(reader)
    }
  }

  pub fn parse(&mut self) -> ParserResult {
    match try!(self.lexer.next_token()) {
      Some(box Token::OpenParen) => {
        self.parse_tail()
      },
      _ => Ok(box Node::Nil)
    }
  }
  
  fn parse_tail(&mut self) -> ParserResult {
    let token = try!(self.lexer.next_token());
    match token {
      Some(box Token::OpenParen) => {
        let left = try!(self.parse_tail());
        let right = try!(self.parse_tail());
        Ok(box Node::Cons(left, right))
      },
      Some(box Token::CloseParen) => {
        Ok(box Node::Nil)
      },
      Some(box Token::Bool(value)) => {
        let left = box Node::Bool(value);
        let right = try!(self.parse_tail());
        Ok(box Node::Cons(left, right))
      },
      Some(box Token::Identifier(name)) => {
        let left = box Node::Atom(name);
        let right = try!(self.parse_tail());
        Ok(box Node::Cons(left, right))
      },
      Some(box Token::Integer32(value)) => {
        let left = box Node::Integer32(value);
        let right = try!(self.parse_tail());
        Ok(box Node::Cons(left, right))
      },
      Some(box Token::U8String(value)) => {
        let left = box Node::U8String(value);
        let right = try!(self.parse_tail());
        Ok(box Node::Cons(left, right))
      },
      None => Ok(box Node::Nil)
    }
  }
}

#[cfg(test)]
fn assert_parse_tree(input: &str, output: &str) {
  let ast = Parser::new(Cursor::new(input.as_bytes()))
    .parse();
  let formatted = format!("{:?}", ast.unwrap());
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
fn test_bool() {
  assert_parse_tree("(true)", "Cons(true, Nil)");
  assert_parse_tree("(false)", "Cons(false, Nil)");
}

#[test]
fn test_parse_function() {
  assert_parse_tree("(abort)", "Cons(:abort, Nil)");
}

#[test]
fn test_parse_string_literal() {
  assert_parse_tree("(println \"string\")", "Cons(:println, Cons(\"string\", Nil))");
}

#[test]
fn test_parse_nested_function() {
  assert_parse_tree(
    "(println (conj 1 2))",
    "Cons(:println, Cons(Cons(:conj, Cons(1_i32, Cons(2_i32, Nil))), Nil))");
}
