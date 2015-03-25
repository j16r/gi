use std::old_path::Path;
use std::old_io::{File, BufferedReader, IoError};
use std::error::FromError;

use parser::{Parser, ParserError};
use environment::Environment;

pub struct Loader {
  environment: Box<Environment>
}

pub struct LoaderError {
  pub explanation : String
}

pub type LoaderResult = Result<Vec<String>, LoaderError>;

impl Loader {
  pub fn new() -> Box<Loader> {
    box Loader {environment: Environment::new()}
  }

  pub fn load(&mut self, files: &[String]) -> LoaderResult {
    let mut outputs = Vec::<String>::new();

    for filename in files {
      println!("loading {:?}", &filename);

      let path = &Path::new(&filename);
      let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(LoaderError {
            explanation: format!("Failed to open file {}", filename)})
      };

      let reader = BufferedReader::new(file);
      let mut parser = Parser::new(reader);
      let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(error) => return Err(LoaderError {
            explanation: format!("Parser error:\n{}:{:?}", filename, error)})
      };

      let result = self.environment.eval(&ast);
      outputs.push(format!("{:?}", result))
    }

    Ok(outputs)
  }
}
