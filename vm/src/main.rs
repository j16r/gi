#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(libc)]

#![feature(collections)]
#![feature(str_char)]

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate libc;

use std::io::{Cursor, Read};
use libc::*;

use loader::Loader;

mod loader;
mod parser;
mod environment;
mod ast;

fn run(files: &[String]) {
    let mut loader = Loader::new();
    loader.load(files);
}

fn exec(input: String) {
    let mut loader = Loader::new();
    loader.exec(Cursor::new(input.as_bytes()));
}

fn print_usage() {
    print!("gi is the frontend for the gi language and framework\n\
          usage: gi <command> ...\n\
          \n\
          \trun <filename>\tRun some gi source code\n");
}

#[cfg(unix)]
fn stdio_isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

fn main() {
    let args: Vec<_> = ::std::env::args().collect();

    if !stdio_isatty() {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input);
        return exec(input);
    }

    if args.len() < 2 {
        print_usage()
    } else {
        match &args[1][..] {
            "run" => run(&args[2 .. args.len()]),
            _     => print_usage(),
        }
    }
}
