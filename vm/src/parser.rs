use std::fmt;
use std::io::{BufferedReader, IoResult, IoError, EndOfFile};

#[cfg(test)]
use std::io::MemReader;

use ast::Node;
use grammar::Token;

pub struct Parser<R> {
  reader: BufferedReader<R>,
  current_char: Option<char>,
  line_number: uint,
  column: uint
}

pub struct ParseError {
  line_number: uint,
  column: uint,
  explanation: String
}

impl fmt::Show for ParseError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}:{} {}",
           self.line_number,
           self.column,
           self.explanation)
  }
}

type ParserResult = Result<Box<Node>, ParseError>;
type LexerResult = Result<Option<Box<Token>>, ParseError>;

impl<R: Reader> Parser<R> {
  pub fn new(reader: R) -> Parser<R> {
    Parser {
      reader: BufferedReader::new(reader),
      current_char: None,
      line_number: 0,
      column: 0
    }
  }

  pub fn parse(&mut self) -> ParserResult {
    match try!(self.next_token()) {
      Some(box Token::OpenParen) => {
        self.parse_tail()
      },
      _ => Ok(box Node::Nil)
    }
  }
  
  fn parse_tail(&mut self) -> ParserResult {
    let token = try!(self.next_token());
    match token {
      Some(box Token::OpenParen) => {
        let left = try!(self.parse_tail());
        let right = try!(self.parse_tail());
        Ok(box Node::Cons(left, right))
      },
      Some(box Token::CloseParen) => {
        Ok(box Node::Nil)
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
      None => Ok(box Node::Nil)
    }
  }

  fn consume_whitespace(&mut self) -> IoResult<char> {
    if self.current_char.is_some() {
      return Ok(self.current_char.unwrap());
    }

    loop {
      match self.reader.read_char() {
        Ok(ch) if ch.is_whitespace() => (),
        Ok(ch) => {
          self.current_char = Some(ch);
          return Ok(ch)
        },
        Err(e) => return Err(e)
      }
    }
  }

  fn consume_token(&mut self) -> LexerResult {
    let mut token = String::from_char(1, self.current_char.unwrap());

    loop {
      match self.reader.read_char() {
        Ok(ch) if !ch.is_whitespace() && ch != ')' => token.push(ch),
        Ok(_) => {
          self.current_char = None;
          return Ok(Some(box Token::Identifier(token)))
        },
        Err(error) => return Err(ParseError {
          line_number: self.line_number,
          column: self.column,
          explanation: format!("{}", error)
        })
      }
    }
  }

  fn consume_i32(&mut self) -> LexerResult {
    let mut token = String::from_char(1, self.current_char.unwrap());

    loop {
      match self.reader.read_char() {
        Ok(ch) if ch.is_digit() => token.push(ch),
        Ok(_) => {
          self.current_char = None;
          return Ok(Some(box Token::Integer32(from_str(token.as_slice()).unwrap())))
        },
        Err(error) => return Err(ParseError {
          line_number: self.line_number,
          column: self.column,
          explanation: format!("{}", error)
        })
      }
    }
  }

  fn next_token(&mut self) -> LexerResult {
    match self.consume_whitespace() {
      Err(IoError { kind: EndOfFile, .. }) | Ok(_) => (),
      Err(error) => return Err(ParseError {
        line_number: self.line_number,
        column: self.column,
        explanation: format!("{}", error)
      })
    }

    match self.current_char {
      Some('(') => {
        self.current_char = None;
        Ok(Some(box Token::OpenParen))
      },
      Some(')') => {
        self.current_char = None;
        Ok(Some(box Token::CloseParen))
      },
      Some(ch) if ch.is_digit() => self.consume_i32(),
      Some(_) => self.consume_token(),
      None => Ok(None)
    }
  }
}

#[cfg(test)]
fn assert_parse_tree(input : &str, output : &str) {
  let ast = Parser::new(MemReader::new(input.as_bytes().to_vec()))
    .parse();
  let formatted = ast.unwrap().to_string();
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
    "Cons(println, Cons(Cons(conj, Cons(1_i32, Cons(2_i32, Nil))), Nil))");
}
