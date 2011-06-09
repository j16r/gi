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

  giClass::giClassPtr instance(ArgumentList &);
  giClass::giClassPtr instance(int32_t);

  void constructor(ArgumentList &);

  inline void value(int32_t value) { _value = value; };
  inline int32_t & value() { return _value; };

  giClass::giClassPtr func_add(ArgumentList & args);
  giClass::giClassPtr func_multiply(ArgumentList & args);
  giClass::giClassPtr func_divide(ArgumentList & args);
  giClass::giClassPtr func_subtract(ArgumentList & args);
  giClass::giClassPtr func_factorial(ArgumentList & args);
  giClass::giClassPtr func_modulus(ArgumentList & args);
};

#endif // _INTEGER_H_
