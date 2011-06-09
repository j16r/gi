#include "includes.H"

giInteger::giInteger() : giClass(GI_INTEGER, __FILE__), _value(0) {
}

giInteger::~giInteger() {
}

giClass::giClassPtr giInteger::instance(ArgumentList & args) {
  giClassPtr new_instance(new giInteger());
  //new_instance->constructor(args);
  return new_instance;
}

giClass::giClassPtr giInteger::instance(int32_t value) {
  giClassPtr new_instance(new giInteger());
  boost::dynamic_pointer_cast<giInteger>(new_instance)->value(value);
  return new_instance;
}

void giInteger::constructor(ArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;
  _value = boost::dynamic_pointer_cast<giInteger>(args.value("value"))->value();
}
