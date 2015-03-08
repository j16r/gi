use std::old_path::Path;
use std::old_io::File;
use std::old_io::BufferedReader;

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
      println!("loading {:?}", &filename);

      let path = &Path::new(&filename);
      let file = match File::open(path) {
          Ok(file) => file,
          Err(error) => {
              println!("Failed to open file {}", filename);
              return
          }
      };

      let reader = BufferedReader::new(file);
      let mut parser = Parser::new(reader);
      let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(error) => {
          println!("Parser error:\n{}:{:?}", filename, error);
          return
        }
      };

      self.environment.eval(&ast);
    }
  }
}
