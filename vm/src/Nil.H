#ifndef _NIL_H_
#define _NIL_H_

#include "Class.H"

class giNil : public giClass {

public:

  giNil() : giClass("Nil", __FILE__) {};

  giClassPtr instance(giClass::giArgumentList & args) {
    std::cout << "Getting singleton Nil instance" << std::endl;
    return giClassPtr(this);
  };
};

#endif // _FILE_H_

