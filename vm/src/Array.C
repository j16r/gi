#include "Engine.H"
#include "Array.H"

giArray::giArray() : giClass("Array", __FILE__) {
  _methods["push"] = (giClass::giMethod)&giArray::push;
  _methods["pop"] = (giClass::giMethod)&giArray::pop;
}

giArray::~giArray() {

}

giClass::ObjectPtr giArray::instance(giClass::giArgumentList & args) {
  ObjectPtr new_instance(new giArray());
  new_instance->constructor(args);
  return new_instance;
}

void giArray::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

}

giClass::ObjectPtr giArray::push(giClass::giArgumentList & args) {
  std::cout << name() << " read" << std::endl;
  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}

giClass::ObjectPtr giArray::pop(giClass::giArgumentList & args) {
  std::cout << name() << " write" << std::endl;
  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}
