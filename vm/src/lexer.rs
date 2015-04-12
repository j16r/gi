use std::fmt;
use std::error::Error;
use std::io::{self, Read, BufReader, Chars, CharsError};

#[cfg(test)]
use std::io::Cursor;

use grammar::Token;

pub struct Lexer<R> {
    characters: Chars<BufReader<R>>,
    current_char: Option<char>,
    line_number: usize,
    column: usize
}

pub struct LexerError {
    pub line_number: usize,
    pub column: usize,
    pub explanation: String
}

pub type LexerResult = Result<Option<Box<Token>>, LexerError>;

impl fmt::Display for LexerError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}:{} {}",
               self.line_number,
               self.column,
               self.explanation)
    }
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Lexer<R> {
        let reader = BufReader::new(reader);
        Lexer {
            characters: reader.chars(),
            current_char: None,
            line_number: 0,
            column: 0
        }
    }

    fn read_char(&mut self) -> Option<Result<char, CharsError>> {
        self.characters.next()
    }

    fn consume_whitespace(&mut self) -> Result<Option<char>, CharsError> {
        if self.current_char.is_some() {
            return Ok(self.current_char);
        }

        loop {
            match self.read_char() {
                Some(Ok(ch)) if ch.is_whitespace() => (),
                Some(Ok(ch)) => {
                    self.current_char = Some(ch);
                    return Ok(Some(ch))
                },
                None => return Ok(None),
                Some(Err(e)) => return Err(e)
            }
        }
    }

    fn consume_token(&mut self) -> LexerResult {
        let mut token = self.current_char.unwrap().to_string();

        loop {
            match self.read_char() {
                Some(Ok(ch)) if !ch.is_whitespace() && ch != ')' =>
                    token.push(ch),
                Some(Ok(_)) | None => {
                    self.current_char = None;
                    if token == "true" {
                        return Ok(Some(Box::new(Token::Bool(true))))
                    } else if token == "false" {
                        return Ok(Some(Box::new(Token::Bool(false))))
                    } else {
                        return Ok(Some(Box::new(Token::Identifier(token))))
                    }
                },
                Some(Err(error)) => return self.lexer_error(error.description())
            }
        }
    }

    fn consume_i32(&mut self) -> LexerResult {
        let mut token = self.current_char.unwrap().to_string();

        loop {
            match self.read_char() {
                Some(Ok(ch)) if ch.is_digit(10) => token.push(ch),
                Some(Ok(_)) | None => {
                    self.current_char = None;
                    let token = Token::Integer32(token.parse().unwrap());
                    return Ok(Some(Box::new(token)))
                },
                Some(Err(error)) =>
                    return self.lexer_error(&error.to_string()[..])
            }
        }
    }

    fn consume_string_literal(&mut self) -> LexerResult {
        let mut token = String::new();

        loop {
            match self.read_char() {
                Some(Ok('"')) => {
                    self.current_char = None;
                    return Ok(Some(Box::new(Token::U8String(token))))
                },
                Some(Ok('\\')) => {
                    match self.read_char() {
                        Some(Ok(ch)) => token.push(ch),
                        _ => panic!("Uh oh!")
                    }
                },
                Some(Ok(ch)) => token.push(ch),
                None => {
                    return Ok(Some(Box::new(Token::U8String(token))))
                },
                Some(Err(error)) =>
                    return self.lexer_error(&error.to_string()[..])
            }
        }
    }

    fn lexer_error(&self, explanation: &str) -> LexerResult {
        Err(LexerError {
            line_number: self.line_number,
            column: self.column,
            explanation: format!("{}", explanation)
        })
    }

    pub fn next_token(&mut self) -> LexerResult {
        match self.consume_whitespace() {
            Ok(_) => (),
            Err(error) => return self.lexer_error(&error.to_string()[..])
        }

        match self.current_char {
            Some('(') => {
                self.current_char = None;
                Ok(Some(Box::new(Token::OpenParen)))
            },
            Some(')') => {
                self.current_char = None;
                Ok(Some(Box::new(Token::CloseParen)))
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
        let next_token = Lexer::new(Cursor::new($input.as_bytes()))
            .next_token();

        match next_token {
            $expected => (),
            Ok(token) =>
                panic!("Expected token \"{:?}\", got {:?}", $input, token),
            Err(error) =>
                panic!("Failed to parse \"{:?}\", got error {}", $input, error)
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
fn test_parse_bool() {
    assert_next_token!("true", Ok(Some(box Token::Bool(true))));
    assert_next_token!("false", Ok(Some(box Token::Bool(false))));
}

#[test]
fn test_parse_string_literal() {
    let next_token = Lexer::new(Cursor::new("\"s\"".as_bytes()))
        .next_token();

    match next_token {
        Ok(Some(box Token::U8String(ref exp))) if *exp == "s".to_string() => (),
        Ok(token) => panic!("Expected token \"?\", got {:?}", token),
        Err(error) => panic!("Failed to parse \"s\", got token {}", error)
    }

    let next_token = Lexer::new(Cursor::new("\"s \\\"s\\\"\"".as_bytes()))
        .next_token();

    match next_token {
        Ok(Some(box Token::U8String(ref exp))) 
            if *exp == "s \"s\"".to_string() => (),
        Ok(token) => panic!("Expected token \"?\", got {:?}", token),
        Err(error) => panic!("Failed to parse \"s\", got token {}", error)
    }
}
