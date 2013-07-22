#[ desc = "gi-lang" ];
#[ license = "MIT" ];
#[ author = "John Barker" ];

extern mod std;

#[path = "src"]
mod vm {
  mod instruction;
  mod loader;
  mod engine;
  mod main;
}
