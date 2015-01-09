use std::fmt;
use std::io::{BufferedReader, IoResult, IoError, EndOfFile};

#[cfg(test)]
use std::io::MemReader;

use grammar::Token;

pub struct Lexer<R> {
  reader: BufferedReader<R>,
  current_char: Option<char>,
  line_number: uint,
  column: uint
}

pub struct LexerError {
  pub line_number: uint,
  pub column: uint,
  pub explanation: String
}

pub type LexerResult = Result<Option<Box<Token>>, LexerError>;

impl fmt::Show for LexerError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}:{} {}",
           self.line_number,
           self.column,
           self.explanation)
  }
}

impl<R: Reader> Lexer<R> {
  pub fn new(reader: R) -> Lexer<R> {
    Lexer {
      reader: BufferedReader::new(reader),
      current_char: None,
      line_number: 0,
      column: 0
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
    let mut token = self.current_char.unwrap().to_string();

    loop {
      match self.reader.read_char() {
        Ok(ch) if !ch.is_whitespace() && ch != ')' => token.push(ch),
        Ok(_) => {
          self.current_char = None;
          return Ok(Some(box Token::Identifier(token)))
        },
        Err(error) => return Err(LexerError {
          line_number: self.line_number,
          column: self.column,
          explanation: format!("{}", error)
        })
      }
    }
  }

  fn consume_i32(&mut self) -> LexerResult {
    let mut token = self.current_char.unwrap().to_string();

    loop {
      match self.reader.read_char() {
        Ok(ch) if ch.is_digit(10) => token.push(ch),
        Ok(_) | Err(IoError { kind: EndOfFile, .. }) => {
          self.current_char = None;
          return Ok(Some(box Token::Integer32(token.parse().unwrap())))
        },
        Err(error) => return Err(LexerError {
          line_number: self.line_number,
          column: self.column,
          explanation: format!("{}", error)
        })
      }
    }
  }

  fn consume_string_literal(&mut self) -> LexerResult {
    let mut token = String::new();

    loop {
      match self.reader.read_char() {
        Ok('"') => {
          self.current_char = None;
          return Ok(Some(box Token::U8String(token)))
        },
        Ok(ch) => token.push(ch),
        Err(error) => return Err(LexerError {
          line_number: self.line_number,
          column: self.column,
          explanation: format!("{}", error)
        })
      }
    }
  }

  pub fn next_token(&mut self) -> LexerResult {
    match self.consume_whitespace() {
      Err(IoError { kind: EndOfFile, .. }) | Ok(_) => (),
      Err(error) => return Err(LexerError {
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
      Some(ch) if ch.is_digit(10) || ch == '-' => self.consume_i32(),
      Some('"') => self.consume_string_literal(),
      Some(_) => self.consume_token(),
      None => Ok(None)
    }
  }
}

macro_rules! assert_next_token {
  ($input:expr, $expected:pat) => ({
    let next_token = Lexer::new(MemReader::new($input.as_bytes().to_vec()))
      .next_token();

    match next_token {
      $expected => (),
      result => panic!("Failed to parse \"{}\", got token {}", $input, result)
    }
  })
}

#[test]
fn test_parse_empty_program() {
  assert_next_token!("", Ok(None));
}

#[test]
fn test_parse_open_paren() {
  assert_next_token!("(", Ok(Some(box Token::OpenParen)));
}

#[test]
fn test_parse_close_paren() {
  assert_next_token!(")", Ok(Some(box Token::CloseParen)));
}

#[test]
fn test_parse_int32() {
  assert_next_token!("3298", Ok(Some(box Token::Integer32(3298))));
  assert_next_token!("-42", Ok(Some(box Token::Integer32(-42))));
}

#[test]
fn test_parse_string_literal() {
  let next_token = Lexer::new(MemReader::new("\"s\"".as_bytes().to_vec()))
      .next_token();

  match next_token {
    Ok(Some(box Token::U8String(ref exp))) if *exp == "s".to_string() => (),
    result => panic!("Failed to parse \"s\", got token {}", result)
  }
}
