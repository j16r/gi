#include "Engine.H"
#include "Exception.H"

giException::giException() : giClass("Exception", __FILE__) {
}

giException::~giException() {
}

void giException::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

  static giArgumentList required;
  required["class"] = engine.lookup_class("Class");
  required["filename"] = engine.lookup_class("String");
  required["message"] = engine.lookup_class("String");

  giClass::check_arguments(required, args);
}
