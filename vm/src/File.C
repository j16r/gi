#include "Engine.H"
#include "File.H"
#include "Nil.H"

giFile::giFile() : giClass("File", __FILE__) {
  _methods["read"] = (giClass::giMethod)&giFile::read;
  _methods["write"] = (giClass::giMethod)&giFile::write;
}

giClass::ObjectPtr giFile::instance(giClass::giArgumentList & args) {
  ObjectPtr new_instance(new giFile());
  new_instance->constructor(args);
  return new_instance;
}

void giFile::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

}

giClass::ObjectPtr giFile::read(giClass::giArgumentList & args) {
  std::cout << name() << " read" << std::endl;

  giClass::giArgumentList::iterator it = args.find("size");
  if(it == args.end()) {
    // throw argument missing error
  }
  // Rudimentary type verification
  if(it->second->name() != "Array") {
    // throw type mismatch error
  }

  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}

giClass::ObjectPtr giFile::write(giClass::giArgumentList & args) {
  std::cout << name() << " write" << std::endl;
  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}
