#ifndef _NIL_H_
#define _NIL_H_

#include "Class.H"
#include "Gi.H"

class giNil : public giClass {

public:

  giNil() : giClass(GI_NIL, __FILE__) {};

  giClassPtr instance(giClass::giArgumentList & args) {
    std::cout << "Getting singleton Nil instance" << std::endl;
    return giClassPtr(this);
  };
};

#endif // _FILE_H_

