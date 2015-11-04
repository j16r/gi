#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(libc)]

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate libc;

use std::io::{Cursor, Read};
use libc::*;

use loader::Loader;

mod ast;
mod environment;
mod functions;
mod lib;
mod loader;
mod parser;

fn run(files: &[String]) {
    let mut loader = Loader::new();
    if let Err(error) = loader.load(files) {
        println!("Failed to execute program on input: {}", error.explanation);
    }
}

fn exec(input: &String) {
    let mut loader = Loader::new();
    if let Err(error) = loader.exec(Cursor::new(input.as_bytes())) {
        println!("Failed to execute program on stdin: {}", error.explanation);
    }
}

fn print_usage() {
    print!("gi is the frontend for the gi language and framework\nusage: gi <command> \
            ...\n\n\trun <filename>\tRun some gi source code\n");
}

#[cfg(unix)]
fn stdio_isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

fn main() {
    let args: Vec<_> = ::std::env::args().collect();

    if !stdio_isatty() {
        let mut input = String::new();
        if let Err(error) = std::io::stdin().read_to_string(&mut input) {
            println!("Failed to read stdin: {}", error);
        }
        return exec(&input);
    }

    if args.len() < 2 {
        print_usage()
    } else {
        match &args[1][..] {
            "run" => run(&args[2..args.len()]),
            _ => print_usage(),
        }
    }
}
