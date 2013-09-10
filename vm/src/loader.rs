use std::io;
use std::path;
pub use vm::parser::*;
pub use vm::environment::*;

struct Loader {
  parser: ~Parser,
  environment: ~Environment
}

impl Loader {
  pub fn new() -> ~Loader {
    println("new()");
    ~Loader{
      parser: Parser::new(),
      environment: Environment::new()}
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
      println(fmt!("loading %s", *filename));
      let lines = self.read(*filename);
      for line in lines.iter() {
        println("line()");
        self.environment.eval(self.parser.parse(*line));
      }
    }
  }
}
