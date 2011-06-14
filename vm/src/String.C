#include "includes.H"

giString::giString() : giClass(GI_STRING, __FILE__) {
}

giString::~giString() {
}

giClass::giClassPtr giString::instance(giArgumentList & args) {
  giClassPtr new_instance(new giString());
  //new_instance->constructor(args);
  return new_instance;
}

giClass::giClassPtr giString::instance(const std::string & value) {
  giClassPtr new_instance(new giString());
  boost::dynamic_pointer_cast<giString>(new_instance)->value(value);
  return new_instance;
}

void giString::constructor(giArgumentList & args) {
  boost::shared_ptr<giString> string = boost::dynamic_pointer_cast<giString>(args.value("value"));
  _value = string->value();
}
