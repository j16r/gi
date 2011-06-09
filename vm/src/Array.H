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

  giClass::giClassPtr instance(ArgumentList & args);

  void constructor(ArgumentList & args);
  giClass::giClassPtr push(ArgumentList & args);
  giClass::giClassPtr pop(ArgumentList & args);

  std::vector<giClassPtr>& data();
};


#endif // _ARRAY_H_
