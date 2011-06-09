#include "includes.H"

giFile::giFile() : giClass(GI_FILE, __FILE__) {
  //_methods["read"] = (giClass::giMethod)&giFile::read;
  //_methods["write"] = (giClass::giMethod)&giFile::write;
}

giClass::giClassPtr giFile::instance(giClass::ArgumentList & args) {
  giClassPtr new_instance(new giFile());
  //new_instance->constructor(args);
  return new_instance;
}

void giFile::constructor(giClass::ArgumentList & args) {
  std::cout << name() << " constructor" << std::endl;
  _path = boost::dynamic_pointer_cast<giString>(args.value("path"))->value();
}

giClass::giClassPtr giFile::read(giClass::ArgumentList & args) {
  std::cout << name() << " read" << std::endl;

  int size = 0;
  giClass::giClassPtr size_arg = args.value("size");
  if(size_arg == NIL) {

    // No argument passed in, read whole file
    size = boost::filesystem::file_size(_path);

  } else if(size_arg == ENGINE->lookup_class(GI_INTEGER)) {

    size = boost::dynamic_pointer_cast<giInteger>(size_arg)->value();
  }

  std::cout << "Reading " << size << " bytes" << std::endl;

  giClass::giClassPtr bytes(new giString());
  boost::dynamic_pointer_cast<giString>(bytes)->value().reserve(size);
  _file.read(&(boost::dynamic_pointer_cast<giString>(bytes)->value()[0]), size);

  return bytes;
}

giClass::giClassPtr giFile::write(giClass::ArgumentList & args) {
  std::cout << name() << " write" << std::endl;

  return NIL;
}
