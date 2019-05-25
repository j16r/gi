use std::fmt;
use std::io::{self, Read};
use lalrpop_util;
use grammar;

#[cfg(test)]
use std::io::Cursor;

use ast::Node;

pub struct Parser<R> {
    reader: R,
}

type ParseError<'input> = lalrpop_util::ParseError<usize, (usize, &'input str), ()>;

pub enum ParserError {
    SyntaxError(String),
    IoError(io::Error),
}

type ParserResult = Result<Box<Node>, ParserError>;

impl From<io::Error> for ParserError {
    fn from(error: io::Error) -> Self {
        ParserError::IoError(error)
    }
}

impl<'input> From<ParseError<'input>> for ParserError {
    fn from(error: ParseError<'input>) -> Self {
        ParserError::SyntaxError(format!("{:?}", error))
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            ParserError::SyntaxError(ref error) => {
                format!("{}", error)
            }
            ParserError::IoError(ref error) => {
                format!("{:?}", error)
            }
        };
        formatter.write_str(&output)
    }
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Parser<R> {
        Parser { reader: reader }
    }

    pub fn parse(&mut self) -> ParserResult {
        let mut input = String::new();
        try!(self.reader.read_to_string(&mut input));
        match grammar::parse_Program(&input) {
            Ok(result) => Ok(result),
            Err(error) => Err(ParserError::from(error)),
        }
    }
}

#[cfg(test)]
fn assert_parse_tree(input: &str, output: &str) {
    let ast = Parser::new(Cursor::new(input.as_bytes())).parse();
    let formatted = format!("{:?}", ast.unwrap());
    assert_eq!(formatted, output.to_string());
}

//#[test]
//fn test_parse_empty_program() {
    //assert_parse_tree("", "Nil");
//}

//#[test]
//fn test_parse_empty_list() {
    //assert_parse_tree("()", "Nil");
    //assert_parse_tree("( )", "Nil");
//}

//#[test]
//fn test_bool() {
    //assert_parse_tree("true", "true");
    //assert_parse_tree("false", "false");
    //assert_parse_tree("(true)", "(true, Nil)");
    //assert_parse_tree("(false)", "(false, Nil)");
//}

//#[test]
//fn test_parse_string_literal() {
    //assert_parse_tree("\"hello\"", "\"hello\"");
    //assert_parse_tree("\"nested \\\"string\\\"\"", "\"nested \\\"string\\\"\"");
    //assert_parse_tree("(println \"string\")",
                      //"(:println, (\"string\", Nil))");
//}

//#[test]
//fn test_parse_integer() {
    //assert_parse_tree("1", "1_i32");
    //assert_parse_tree("10", "10_i32");
    //assert_parse_tree("0", "0_i32");
//}

//#[test]
//fn test_parse_nested_function() {
    //assert_parse_tree("println(conj(1 2))",
                      //"(:println, ((:conj, (1_i32, (2_i32, Nil))), Nil))");
//}

//#[test]
//fn test_value_after_function() {
    //assert_parse_tree("println((add 1 2) 3)",
                      //"(:println, ((:add, (1_i32, (2_i32, Nil))), \
                       //(3_i32, Nil)))");
//}

//#[test]
//fn test_multiple_expressions() {
    //assert_parse_tree("(println 3) (println 9)",
                      //"((:println, (3_i32, Nil)), (:println, (9_i32, Nil)))");
    //assert_parse_tree("(println 7)(println 11)",
                      //"((:println, (7_i32, Nil)), (:println, (11_i32, Nil)))");
//}

#[test]
fn test_function_application_expression() {
    assert_parse_tree("println(3)",
                      "println((3_i32, Nil))");
    assert_parse_tree("println(3) println(9)",
                      "(println((3_i32, Nil)), println((9_i32, Nil)))");
}
