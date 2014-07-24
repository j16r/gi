use std::io::File;
use std::path::{Path};
use vm::parser::parse;
use vm::environment::{Environment, dump_token};

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
      let contents_result = file.read_to_string();

      match contents_result {
        Ok(contents) => {
          let ast = parse(contents);
          let env = self.environment.eval(&ast);
          let root_token = dump_token(env);
          println!(">\n{:s}", root_token.as_slice());
        },
        Err(_) => fail!("Error opening file")
      }
    }
  }
}
