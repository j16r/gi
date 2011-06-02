#include "Engine.H"
#include "Gi.H"

giInteger::giInteger() : giClass(GI_INTEGER, __FILE__), _value(0) {
}

giInteger::~giInteger() {
}

giClass::giClassPtr giInteger::instance(giClass::giArgumentList & args) {
  giClassPtr new_instance(new giInteger());
  new_instance->constructor(args);
  return new_instance;
}

giClass::giClassPtr giInteger::instance(int32_t value) {
  giClassPtr new_instance(new giInteger());
  boost::dynamic_pointer_cast<giInteger>(new_instance)->value(value);
  return new_instance;
}

void giInteger::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

  static giArgumentList required;
  required["value"] = engine.lookup_class(GI_INTEGER);

  giClass::check_arguments(required, args);
  _value = boost::dynamic_pointer_cast<giInteger>(args["value"])->value();
}
