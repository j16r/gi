#![ desc = "gi-lang" ]
#![ license = "MIT" ]
#![ author = "John Barker" ]
#![feature(macro_rules)]

#[path = "src"]
mod vm {
  mod loader;
  mod parser;
  mod environment;
  mod ast;

  mod main;
}
