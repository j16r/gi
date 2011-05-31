#ifndef _NIL_H_
#define _NIL_H_

#include "Class.H"

class giNil : public giClass {

public:

  giNil() : giClass("Nil", __FILE__) {};

  ObjectPtr instance(giClass::giArgumentList & args) {
    std::cout << "Getting singleton Nil instance" << std::endl;
    return ObjectPtr(this);
  };
};

#endif // _FILE_H_

