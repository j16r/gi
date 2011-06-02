#ifndef _INTEGER_H_
#define _INTEGER_H_

#include "Class.H"

class giInteger : public giClass {

  int32_t _value;

public:

  giInteger();
  ~giInteger();

  giClass::giClassPtr instance(giClass::giArgumentList &);
  giClass::giClassPtr instance(int32_t);

  void constructor(giClass::giArgumentList &);

  inline void value(int32_t value) { _value = value; };
  inline int32_t & value() { return _value; };
};

#endif // _INTEGER_H_
