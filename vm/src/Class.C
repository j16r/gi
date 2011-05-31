#include <string>

#include "Class.H"
#include "Engine.H"

#include <boost/filesystem.hpp>
#include <boost/filesystem/fstream.hpp>
#include <iostream>
#include <string>
#include <list>
namespace fs = boost::filesystem;

giClass::giClass() :
  _name("Class"),
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
  giClassPtr new_instance(new giClass("Class", __FILE__));
  new_instance->invoke("constructor", args);
  return new_instance;
}

giClass::giClassPtr giClass::invoke(const std::string & method_name, giClass::giArgumentList & args) {
  std::cout << "Invoking " << method_name << " with " << args.size() << " arguments" << std::endl;
  giMethod func = _methods[method_name];
  std::cout << "Function ptr " << func << std::endl;
  return (this->*func)(args);
}
