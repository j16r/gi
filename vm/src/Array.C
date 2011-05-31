#include "Engine.H"
#include "Array.H"

giArray::giArray() : giClass("Array", __FILE__) {
  _methods["push"] = (giClass::giMethod)&giArray::push;
  _methods["pop"] = (giClass::giMethod)&giArray::pop;
}

giArray::~giArray() {

}

giClass::giClassPtr giArray::instance(giClass::giArgumentList & args) {
  giClassPtr new_instance(new giArray());
  new_instance->constructor(args);
  return new_instance;
}

void giArray::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

}

giClass::giClassPtr giArray::push(giClass::giArgumentList & args) {
  std::cout << name() << " read" << std::endl;
  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}

giClass::giClassPtr giArray::pop(giClass::giArgumentList & args) {
  std::cout << name() << " write" << std::endl;
  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}
