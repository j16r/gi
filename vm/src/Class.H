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

  void process(const char *, const std::streamsize&);

public:

  typedef boost::shared_ptr<giClass> giClassPtr;
  typedef std::map<std::string, giClassPtr> giArgumentList;
  typedef giClassPtr (giClass::*giMethod)(giArgumentList & args);
  typedef std::map<std::string, giMethod> MethodMap;

  MethodMap _methods;

  // ctors (internal)
  giClass();
  giClass(const std::string&, const std::string&);
  giClass(const std::string&, const std::string&, fs::ifstream &);

  inline const std::string& name() const { return _name; }

  virtual giClassPtr instance(giArgumentList &);

  giClassPtr invoke(const std::string &, giArgumentList &);

  void constructor(giArgumentList &) {};
  void destructor() {};

  static void check_arguments(const giArgumentList & , const giArgumentList & );
};

#endif // _CLASS_H_
