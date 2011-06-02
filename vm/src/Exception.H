#ifndef _EXCEPTION_H_
#define _EXCEPTION_H_

#include "Class.H"

class giException : public giClass {

  std::string _class;
  std::string _filename;
  std::string _message;

public:

  giException();
  ~giException();

  void constructor(giClass::giArgumentList & args);

  giClassPtr instance(giArgumentList &);
  giClassPtr instance(const std::string &, const std::string &, const std::string &);

  inline void set_class(const std::string & value) { _class = value; }
  inline void set_filename(const std::string & value) { _filename = value; }
  inline void set_message(const std::string & value) { _message = value; }
  inline std::string get_class() { return _class; }
  inline std::string get_filename() { return _filename; }
  inline std::string get_message() { return _message; }

  std::string dump();

  giClass::giClassPtr func_class(giClass::giArgumentList & args);
  giClass::giClassPtr func_file(giClass::giArgumentList & args);
  giClass::giClassPtr func_message(giClass::giArgumentList & args);
};

#endif // _EXCEPTION_H_
