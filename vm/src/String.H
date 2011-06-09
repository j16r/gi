#ifndef _GI_STRING_H_
#define _GI_STRING_H_

class giString : public giClass {

  std::string _value;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giString());
  }

public:

  giString();
  ~giString();

  giClassPtr instance(ArgumentList &);
  giClassPtr instance(const std::string &);

  void constructor(ArgumentList & args);

  inline std::string & value() { return _value; };
  inline const void value(const std::string & value) { _value = value; };
};

#endif // _GI_STRING_H_
