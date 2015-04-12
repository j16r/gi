#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(convert)]

#![feature(collections)]
#![feature(str_char)]

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

use loader::Loader;

mod loader;
mod parser;
mod environment;
mod ast;

fn run(files: &[String]) {
    let mut loader = Loader::new();
    loader.load(files);
}

fn print_usage() {
    print!("gi is the frontend for the gi language and framework\n\
          usage: gi <command> ...\n\
          \n\
          \trun <filename>\tRun some gi source code\n");
}

fn main() {
    let args: Vec<_> = ::std::env::args().collect();

    if args.len() < 2 {
        print_usage()
    } else {
        match &args[1][..] {
            "run" => run(&args[2 .. args.len()]),
            _     => print_usage(),
        }
    }
}
