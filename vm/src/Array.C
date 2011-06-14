#include "includes.H"

giArray::giArray() : giClass(GI_ARRAY, __FILE__) {
  //_methods["push"] = (giClass::giMethod)&giArray::push;
  //_methods["pop"] = (giClass::giMethod)&giArray::pop;
}

giArray::~giArray() {

}

void giArray::constructor(giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;
}

giClass::giClassPtr giArray::push(giArgumentList & args) {
  std::cout << name() << " read" << std::endl;
  return NIL;
}

giClass::giClassPtr giArray::pop(giArgumentList & args) {
  std::cout << name() << " write" << std::endl;
  return NIL;
}
