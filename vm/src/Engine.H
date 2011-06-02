#ifndef _ENGINE_H_
#define _ENGINE_H_

#include <map>
#include <boost/shared_ptr.hpp>

#include "Class.H"

class giEngine : public giClass {

  typedef std::map<std::string, boost::shared_ptr<giClass> > ClassMap;
  ClassMap _classes;

public:

  giEngine();
  ~giEngine();

  void load_class(const std::string &file_name);
  void load_builtin_classes();
  void dump_classes() const;
  giClass::giClassPtr lookup_class(const std::string &class_name);

  giClass::giClassPtr func_lookup_class(giClass::giArgumentList & args);
};

extern giEngine engine;

#endif // _ENGINE_H_
