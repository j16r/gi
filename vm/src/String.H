#ifndef _GI_STRING_H_
#define _GI_STRING_H_

#include "Class.H"

class giString : public giClass {

  std::string _value;

public:

  giString();
  ~giString();

  giClassPtr instance(giArgumentList &);
  giClassPtr instance(const std::string &);

  void constructor(giClass::giArgumentList & args);

  inline const std::string & value() const { return _value; };
  inline const void value(const std::string & value) { _value = value; };
};

#endif // _GI_STRING_H_
