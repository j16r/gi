#ifndef _ENGINE_H_
#define _ENGINE_H_

#include <map>
#include <boost/shared_ptr.hpp>

#include "Class.H"

class giEngine {

  typedef std::map<std::string, boost::shared_ptr<giClass> > ClassMap;
  ClassMap _classes;

public:

  void load_class(const std::string &file_name);
  void load_builtin_classes();
  void dump_classes() const;
  boost::shared_ptr<giClass> lookup_class(const std::string &class_name);

};

extern giEngine engine;

#endif // _ENGINE_H_
