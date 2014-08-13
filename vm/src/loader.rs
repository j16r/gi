use std::io::File;
use std::path::{Path};
use vm::parser::{parse, ParseError};
use vm::environment::{Environment};

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

      let path = Path::new(filename.as_slice());
      let mut file = File::open(&path).unwrap();
      let contents = file.read_to_string();

      match parse(contents.unwrap()) {
        Ok(ast) => { self.environment.eval(&ast); },
        Err(_) => fail!("Parsing error")
      }
    }
  }
}
