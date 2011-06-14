#ifndef _ARRAY_H_
#define _ARRAY_H_

class giArray : public giClass {

  std::vector<giClassPtr> _data;
  
protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giArray());
  }

public:

  giArray();
  ~giArray();

  giClass::giClassPtr instance(giArgumentList & args);

  void constructor(giArgumentList & args);
  giClass::giClassPtr push(giArgumentList & args);
  giClass::giClassPtr pop(giArgumentList & args);

  std::vector<giClassPtr>& data();
};


#endif // _ARRAY_H_
