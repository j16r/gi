use std::io;
use std::os;
use vm::loader::Loader;

fn run(files: &[String]) {
  let mut loader = Loader::new();
  loader.load(files)
}

fn print_usage() {
  io::print("gi is frontend for the gi language and framework\n\
             usage: gi <command> ...\n\
             \n\
             \trun <filename>\tRun some gi source code\n");
}

#[main]
fn main() {
  let os_args = os::args();

  if (os_args.len() < 2) {
    print_usage()
  } else {
    match os_args.get(1).as_slice() {
      "run" => run(os_args.slice(2, os_args.len())),
      _     => print_usage(),
    }
  }
}