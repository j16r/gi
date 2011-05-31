#include "Class.H"
#include "Array.H"
#include "Engine.H"
#include "File.H"

giEngine engine;

int main(int argc, char *argv[]) {
  engine.load_builtin_classes();
  engine.dump_classes();

  giClass::giArgumentList empty;
  engine.lookup_class("File")->invoke("read", empty);
}
