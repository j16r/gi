#ifndef _ARRAY_H_
#define _ARRAY_H_

#include <vector>

#include "Class.H"

class giArray : public giClass {

  std::vector<giClassPtr> _data;

public:

  giArray();
  ~giArray();

  giClass::ObjectPtr instance(giClass::giArgumentList & args);

  void constructor(giClass::giArgumentList & args);
  giClass::ObjectPtr push(giClass::giArgumentList & args);
  giClass::ObjectPtr pop(giClass::giArgumentList & args);

  std::vector<giClassPtr>& data();
};


#endif // _ARRAY_H_
