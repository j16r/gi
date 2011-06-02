#include "Engine.H"
#include "Gi.H"

giException::giException() : giClass(GI_EXCEPTION, __FILE__) {
}

giException::~giException() {
}


giClass::giClassPtr giException::instance(giArgumentList & args) {
  giClassPtr new_instance(new giException());
  new_instance->constructor(args);
  return new_instance;
}

giClass::giClassPtr giException::instance(const std::string & class_name, const std::string & file_name, const std::string & message) {
  giClassPtr new_instance(new giException());
  boost::dynamic_pointer_cast<giException>(new_instance)->set_class(class_name);
  boost::dynamic_pointer_cast<giException>(new_instance)->set_filename(file_name);
  boost::dynamic_pointer_cast<giException>(new_instance)->set_message(message);
  return new_instance;
}

void giException::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

  static giArgumentList required;
  required["class"] = engine.lookup_class(GI_CLASS);
  required["filename"] = engine.lookup_class(GI_STRING);
  required["message"] = engine.lookup_class(GI_STRING);

  giClass::check_arguments(required, args);
}

std::string giException::dump() {
  return std::string("(") + _class + ") " + _filename + ": " + _message;
}
