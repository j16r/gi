#ifndef _EXCEPTION_H_
#define _EXCEPTION_H_

class giException : public giClass {

  giClass::giClassPtr _class;
  std::string _filename;
  std::string _message;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giException());
  }

public:

  giException();
  ~giException();

  void constructor(ArgumentList & args);

  giClassPtr instance(ArgumentList &);
  giClassPtr instance(giClass::giClassPtr value, const std::string &, const std::string &);

  inline void set_class(giClass::giClassPtr value) { _class = value; }
  inline void set_filename(const std::string & value) { _filename = value; }
  inline void set_message(const std::string & value) { _message = value; }
  inline giClass::giClassPtr get_class() { return _class; }
  inline std::string get_filename() { return _filename; }
  inline std::string get_message() { return _message; }

  std::string dump();

  giClass::giClassPtr func_class(ArgumentList & args);
  giClass::giClassPtr func_file(ArgumentList & args);
  giClass::giClassPtr func_message(ArgumentList & args);
};

#endif // _EXCEPTION_H_
