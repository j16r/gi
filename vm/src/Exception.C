#include "Engine.H"
#include "Exception.H"

giException::giException() : giClass("Exception", __FILE__) {
}

giException::~giException() {
}

void giException::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

  static giArgumentList arguments;
  arguments["class"] = engine.lookup_class("Class");
  arguments["filename"] = engine.lookup_class("String");
  arguments["message"] = engine.lookup_class("String");
}
