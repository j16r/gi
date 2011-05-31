#ifndef _EXCEPTION_H_
#define _EXCEPTION_H_

#include "Class.H"

class giException : public giClass {
public:

  giException();
  ~giException();

  void constructor(giClass::giArgumentList & args);

  giClass::giClassPtr Class(giClass::giArgumentList & args);
  giClass::giClassPtr file(giClass::giArgumentList & args);
  giClass::giClassPtr message(giClass::giArgumentList & args);
};

#endif // _EXCEPTION_H_
