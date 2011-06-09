#include "includes.H"

giEngine::giEngine() : giClass(GI_ENGINE, __FILE__) {
  //_slots["lookup_class"] = (giClass::giMethod)&giEngine::func_lookup_class;
}

giEngine::~giEngine() {
}

void giEngine::load_class(
    const std::string &file_name) {
  std::string class_name = file_name.substr(0, file_name.find_last_of("."));

  fs::ifstream class_file(file_name);
  giClassPtr new_class(new giClass(class_name, file_name, class_file));
  _classes[class_name] = new_class;
  std::cout << "Class loaded: " << new_class->name() << std::endl;
}

void giEngine::load_builtin_classes() {
  _classes[GI_ENGINE] = giClassPtr(this);
  _classes[GI_NIL] = giClassPtr(new giNil());
  _classes[GI_CLASS] = giClassPtr(new giClass());
  _classes[GI_ARRAY] = giClassPtr(new giArray());
  _classes[GI_STRING] = giClassPtr(new giString());
  _classes[GI_FILE] = giClassPtr(new giFile());
  _classes[GI_EXCEPTION] = giClassPtr(new giException());
  _classes[GI_INTEGER] = giClassPtr(new giInteger());
}

void giEngine::dump_classes() const {

  std::vector<std::string> class_list;
  for(ClassMap::const_iterator it = _classes.begin(); it != _classes.end(); ++it) {
    class_list.push_back(it->second->name());
  }
  std::cout << "Classes loaded: " << boost::algorithm::join(class_list, ", ") << "." << std::endl;
}

giClass::giClassPtr giEngine::lookup_class(
    const std::string &class_name) {

  ClassMap::iterator it = _classes.find(class_name);
  std::cout << "Looking up class " << class_name << std::endl;

  if(it == _classes.end()) {

    std::cout << "Unable to find class " << class_name << std::endl;
    throw EXCEPTION->instance(
        _c(GI_ENGINE),
        __FILE__,
        std::string("Unable to find class ") + class_name);
  }

  return it->second;
}

giClass::giClassPtr giEngine::func_lookup_class(
    giClass::ArgumentList & args) {

  return lookup_class(boost::dynamic_pointer_cast<giString>(args.value("name"))->value());
}
