#[ desc = "gi-lang" ];
#[ license = "MIT" ];
#[ author = "John Barker" ];

extern mod std;

#[path = "src"]
mod vm {
  mod engine;
  mod instruction;
  mod loader;
  mod parser;
  mod environment;

  mod main;
}
