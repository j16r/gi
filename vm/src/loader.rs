use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};

use parser::Parser;
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
        Box::new(Loader {environment: Environment::new()})
    }

    pub fn load(&mut self, files: &[String]) -> LoaderResult {
        let mut outputs = Vec::<String>::new();

        for filename in files {
            let path = &Path::new(&filename);
            let file = match File::open(path) {
                Ok(file) => file,
                Err(error) => return Err(LoaderError {
                    explanation: format!("Failed to open file {}, reason {}",
                                         filename,
                                         error)})
            };

            let reader = BufReader::new(file);
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

    pub fn exec<R: Read>(&mut self, input: R) -> LoaderResult {
        let mut parser = Parser::new(input);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(error) => return Err(LoaderError {
                explanation: format!("Parser error:\n{:?}", error)})
        };

        let result = self.environment.eval(&ast);
        Ok(vec![format!("{:?}", result)])
    }
}
