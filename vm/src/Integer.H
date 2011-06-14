#ifndef _INTEGER_H_
#define _INTEGER_H_

class giInteger : public giClass {

  int32_t _value;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giInteger());
  }

public:

  giInteger();
  ~giInteger();

  giClass::giClassPtr instance(giArgumentList &);
  giClass::giClassPtr instance(int32_t);

  void constructor(giArgumentList &);

  inline void value(int32_t value) { _value = value; };
  inline int32_t & value() { return _value; };

  giClass::giClassPtr func_add(giArgumentList & args);
  giClass::giClassPtr func_multiply(giArgumentList & args);
  giClass::giClassPtr func_divide(giArgumentList & args);
  giClass::giClassPtr func_subtract(giArgumentList & args);
  giClass::giClassPtr func_factorial(giArgumentList & args);
  giClass::giClassPtr func_modulus(giArgumentList & args);
};

#endif // _INTEGER_H_
