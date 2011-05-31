#ifndef _CLASS_H_
#define _CLASS_H_

#include <boost/filesystem.hpp>
#include <boost/filesystem/fstream.hpp>
#include <boost/shared_ptr.hpp>
#include <vector>
#include <string>
#include <map>
namespace fs = boost::filesystem;

class giClass;
typedef boost::shared_ptr<giClass> giClassPtr;

class giClass {

  const std::string _name;
  const std::string _file_name;

  void process(const char * bytes, const std::streamsize& size);

public:

  typedef boost::shared_ptr<giClass> giClassPtr;
  typedef std::map<std::string, giClassPtr> giArgumentList;
  typedef giClassPtr (giClass::*giMethod)(giArgumentList & args);
  typedef std::map<std::string, giMethod > MethodMap;

  MethodMap _methods;

  giClass();
  giClass(const std::string& name, const std::string& file_name);
  giClass(const std::string& name, const std::string& file_name, fs::ifstream & file);

  inline const std::string& name() const { return _name; }

  virtual giClassPtr instance(giClass::giArgumentList & args);

  giClassPtr invoke(const std::string & method_name, giClass::giArgumentList & args);

  void constructor(giClass::giArgumentList & args) {};
  void destructor() {};
};

#endif // _CLASS_H_
