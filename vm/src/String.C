#include "Engine.H"
#include "String.H"

giString::giString() : giClass("String", __FILE__) {
}

giString::~giString() {
}

giClass::giClassPtr giString::instance(giArgumentList & args) {
  giClassPtr new_instance(new giString());
  new_instance->constructor(args);
  return new_instance;
}

giClass::giClassPtr giString::instance(const std::string & value) {
  giClassPtr new_instance(new giString());
  boost::dynamic_pointer_cast<giString>(new_instance)->value(value);
  return new_instance;
}

void giString::constructor(giClass::giArgumentList & args) {
  giArgumentList::const_iterator argument = args.find("value");

  if(argument != args.end()) {
    if(argument->second == engine.lookup_class("String")) {
      boost::shared_ptr<giString> string = boost::dynamic_pointer_cast<giString>(argument->second);
      _value = string->value();
    }
  }
}
