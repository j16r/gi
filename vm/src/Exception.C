#include "includes.H"

giException::giException() : giClass(GI_EXCEPTION, __FILE__) {
}

giException::~giException() {
}

giClass::giClassPtr giException::instance(
    giClass::giClassPtr the_class,
    const std::string & file_name,
    const std::string & message) {

  giClassPtr new_instance(new giException());
  boost::dynamic_pointer_cast<giException>(new_instance)->set_class(the_class);
  boost::dynamic_pointer_cast<giException>(new_instance)->set_filename(file_name);
  boost::dynamic_pointer_cast<giException>(new_instance)->set_message(message);
  return new_instance;
}

void giException::constructor(ArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;
}

std::string giException::dump() {
  return std::string("(") + _class->name() + ") " + _filename + ": " + _message;
}
