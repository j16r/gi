use std::io;
use std::path;

struct Loader {
  x: bool
}

impl Loader {
  pub fn new() -> ~Loader {
    ~Loader { x: false }
  }

  fn read(self, filename: &str) -> ~[~str] {
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
    for files.iter().advance |filename| {
      let lines = self.read(*filename);
      for lines.iter().advance |line| {
        io::println(*line);
      }
    }
  }
}
