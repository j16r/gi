use std::path::Path;
use std::io::File;
use std::io::BufferedReader;

use parser::Parser;
use environment::Environment;

pub struct Loader {
  environment: Box<Environment>
}

impl Loader {
  pub fn new() -> Box<Loader> {
    box Loader {
      environment: Environment::new()}
  }

  pub fn load(&mut self, files: &[String]) {
    for filename in files.iter() {
      println!("loading {:s}", filename.as_slice());

      let path = &Path::new(filename.as_slice());
      let file = File::open(path).unwrap();
      let reader = BufferedReader::new(file);
      let mut parser = Parser::new(reader);

      let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(error) => {
          println!("Parser error:\n{}:{}", filename, error);
          return
        }
      };

      self.environment.eval(&ast);
    }
  }
}
