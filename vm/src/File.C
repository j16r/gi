#include "Engine.H"
#include "File.H"
#include "Nil.H"
#include "String.H"
#include "Array.H"
#include "Integer.H"

giFile::giFile() : giClass("File", __FILE__) {
  _methods["read"] = (giClass::giMethod)&giFile::read;
  _methods["write"] = (giClass::giMethod)&giFile::write;
}

giClass::giClassPtr giFile::instance(giClass::giArgumentList & args) {
  giClassPtr new_instance(new giFile());
  new_instance->constructor(args);
  return new_instance;
}

void giFile::constructor(giClass::giArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;

  static giArgumentList required;
  required["path"] = engine.lookup_class("String");

  giClass::check_arguments(required, args);
  _path = boost::dynamic_pointer_cast<giString>(args["path"])->value();
}

giClass::giClassPtr giFile::read(giClass::giArgumentList & args) {
  std::cout << name() << " read" << std::endl;

  static giArgumentList required;
  required["size"] = engine.lookup_class("Integer");

  giClass::check_arguments(required, args);

  int size = 0;
  giClass::giClassPtr size_arg = args["size"];
  if(size_arg == engine.lookup_class("Nil")) {

    // No argument passed in, read whole file
    size = boost::filesystem::file_size(_path);

  } else if(size_arg == engine.lookup_class("Integer")) {

    size = boost::dynamic_pointer_cast<giInteger>(size_arg)->value();
  }

  giClass::giClassPtr bytes(new giString());
  boost::dynamic_pointer_cast<giString>(bytes)->value().reserve(size);
  _file.read(&(boost::dynamic_pointer_cast<giString>(bytes)->value()[0]), size);

  return bytes;
}

giClass::giClassPtr giFile::write(giClass::giArgumentList & args) {
  std::cout << name() << " write" << std::endl;
  giClass::giArgumentList empty;
  return engine.lookup_class("Nil")->instance(empty);
}
