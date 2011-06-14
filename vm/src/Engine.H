#ifndef _ENGINE_H_
#define _ENGINE_H_

class giEngine : public giClass {

  typedef std::map<std::string, boost::shared_ptr<giClass> > ClassMap;
  ClassMap _classes;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giEngine());
  }

public:

  giEngine();
  ~giEngine();

  void load_class(const std::string &file_name);
  void load_builtin_classes();
  void dump_classes() const;

  giClassPtr lookup_class(const std::string &class_name);

  giClassPtr func_lookup_class(giArgumentList & args);
};

extern giClass::giClassPtr g_engine;

#endif // _ENGINE_H_
