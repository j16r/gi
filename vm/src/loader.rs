use std::io;
use std::path;
pub use vm::parser::*;
pub use vm::eval::*;
pub use vm::environment::*;

struct Loader {
  parser: ~Parser,
  environment: ~Token
}

impl Loader {
  pub fn new() -> ~Loader {
    ~Loader{
      parser: Parser::new(),
      environment: create_environment()}
  }

  fn read(&self, filename: &str) -> ~[~str] {
    let read_result: Result<@Reader, ~str>;
    read_result = io::file_reader(~path::Path(filename));

    if read_result.is_ok() {
      let file = read_result.unwrap();
      return file.read_lines();
    }

    println(fmt!("Error reading file: %?", read_result.unwrap_err()));
    return ~[];
  }

  pub fn load(&mut self, files: &[~str]) {
    for filename in files.iter() {
      let lines = self.read(*filename);
      for line in lines.iter() {
        eval(self.parser.parse(*line), self.environment);
      }
    }
  }
}
