#ifndef _CLASS_H_
#define _CLASS_H_

#include <boost/filesystem.hpp>
#include <boost/filesystem/fstream.hpp>
#include <boost/shared_ptr.hpp>
#include <vector>
#include <string>
#include <map>
namespace fs = boost::filesystem;

#include "Object.H"

class giClass {

  const std::string _name;
  const std::string _file_name;

  void process(const char * bytes, const std::streamsize& size);

public:

  typedef boost::shared_ptr<giClass> ObjectPtr;
  typedef std::map<std::string, ObjectPtr> giArgumentList;
  typedef ObjectPtr (giClass::*giMethod)(giArgumentList & args);
  typedef std::map<std::string, giMethod > MethodMap;

  MethodMap _methods;

  giClass(const std::string& name, const std::string& file_name);
  giClass(const std::string& name, const std::string& file_name, fs::ifstream & file);

  inline const std::string& name() const { return _name; }

  virtual ObjectPtr instance(giClass::giArgumentList & args);

  ObjectPtr invoke(const std::string & method_name, giClass::giArgumentList & args);

  void constructor(giClass::giArgumentList & args) {};
  void destructor() {};
};

typedef boost::shared_ptr<giClass> giClassPtr;

#endif // _CLASS_H_
