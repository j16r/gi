#include "Class.H"
#include "Engine.H"
#include "Array.H"
#include "File.H"
#include "Nil.H"

void giEngine::load_class(const std::string &file_name) {
  std::string class_name = file_name.substr(0, file_name.find_last_of("."));

  fs::ifstream class_file(file_name);
  boost::shared_ptr<giClass> new_class(new giClass(class_name, file_name, class_file));
  _classes[class_name] = new_class;
  std::cout << "Class loaded: " << new_class->name() << std::endl;
}

void giEngine::load_builtin_classes() {
  _classes["Nil"] = boost::shared_ptr<giClass>(new giNil());
  _classes["Array"] = boost::shared_ptr<giClass>(new giArray());
  _classes["File"] = boost::shared_ptr<giClass>(new giFile());
}

void giEngine::dump_classes() const {
  std::cout << "Classes loaded: ";
  for(ClassMap::const_iterator it = _classes.begin(); it != _classes.end(); ++it) {
    std::cout << it->second->name() << ", ";
  }
  std::cout << std::endl;
}

giClass::ObjectPtr giEngine::lookup_class(const std::string &class_name) {
  ClassMap::iterator it = _classes.find(class_name);
  std::cout << "Looking up class " << class_name << std::endl;
  if(it == _classes.end()) {
    std::cout << "Class with name " << class_name << " not found." << std::endl;
  }
  return it->second;
}
