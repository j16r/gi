#include <string>

#include "Engine.H"
#include "Gi.H"

#include <boost/filesystem.hpp>
#include <boost/filesystem/fstream.hpp>
#include <iostream>
#include <string>
#include <list>
namespace fs = boost::filesystem;

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

giClass::giClassPtr giClass::instance(giClass::giArgumentList & args) {
  giClassPtr new_instance(new giClass(GI_CLASS, __FILE__));
  new_instance->invoke("constructor", args);
  return new_instance;
}

giClass::giClassPtr giClass::invoke(const std::string & method_name, giClass::giArgumentList & args) {
  std::cout << "Invoking " << method_name << " with " << args.size() << " arguments" << std::endl;
  giMethod func = _methods[method_name];
  std::cout << "Function ptr " << func << std::endl;
  return (this->*func)(args);
}

void giClass::check_arguments(const giArgumentList & required_arguments, const giArgumentList & actual_arguments) {
  for(giArgumentList::const_iterator required = required_arguments.begin();
      required != required_arguments.end();
      ++required) {

    giArgumentList::const_iterator actual = actual_arguments.find(required->first);

    // Argument specified like (arg, ...
    if(required->second == engine.lookup_class(GI_NIL)) {
      if(actual == actual_arguments.end()) {
        // no default was specified and no argument was specified

        giArgumentList exception_args;
        exception_args["class"] = engine.lookup_class(GI_CLASS);
        exception_args["filename"] =
          boost::dynamic_pointer_cast<giString>(engine.lookup_class(GI_STRING))->instance(__FILE__);
        throw engine.lookup_class(GI_EXCEPTION)->instance(exception_args);
      }

    // Argument specified like (arg: String, ...
    } else if(actual != actual_arguments.end()) {
      // TODO: needs to traverse up the tree
      if(actual->second != required->second) {
        // type mismatch exception

      }

      

    } else {

    }

  }
}
