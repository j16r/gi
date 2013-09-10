#[ desc = "gi-lang" ];
#[ license = "MIT" ];
#[ author = "John Barker" ];

extern mod std;

#[path = "src"]
mod vm {
  mod loader;
  mod parser;
  mod environment;

  mod main;
}
