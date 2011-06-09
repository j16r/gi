#include "includes.H"

giClass::giClass() :
  _name(GI_CLASS),
  _file_name(__FILE__) {
}

giClass::giClass(const std::string& name, const std::string& file_name) :
  _name(name),
  _file_name(file_name) {
}

giClass::giClass(const std::string& name, const std::string& file_name, fs::ifstream & class_file) :
  _name(name),
  _file_name(file_name) {

  char buffer[256] = {0};
  std::streamsize bytes = sizeof(buffer);
  while(!class_file.eof()) {
    class_file.read(buffer, bytes);
    process(buffer, class_file.gcount());
  }
}

void giClass::process(const char * bytes, const std::streamsize& size) {
  std::cout << "Processing " << size << " bytes of bytecode:" << bytes << std::endl;
}

giClass::giClassPtr giClass::instance(
    giClassPtr sender,
    ArgumentList &args) {

  giClassPtr new_instance(create_instance());

  std::cout << "Creating instance of " << new_instance->name() << std::endl;

  giFunction *function(boost::dynamic_pointer_cast<giFunction>(new_instance->lookup("constructor")).get());
  if(function) {
    std::cout << new_instance->name() << " defines a constructor, invoking..." << std::endl;
    function->invoke(ENGINE->lookup_class(GI_CLASS), NIL, args);
  }

  return new_instance;
}

giClass::giClassPtr giClass::lookup(const std::string & identifier) {
  std::cout << "Looking up identifier " << identifier << " on class " << name() << std::endl;

  Slot::iterator it = _slots.find(identifier);

  if(it == _slots.end()) {

    std::cout << "Unable to find identifier " << identifier << std::endl;
    throw EXCEPTION->instance(
        _c(GI_ENGINE),
        __FILE__,
        std::string("Unable to find identifier ") + identifier);
  }

  return it->second;
}
