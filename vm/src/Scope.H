#ifndef _SCOPE_H_
#define _SCOPE_H_

class giScope : public giClass {

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giScope());
  }

public:


};

#endif // _SCOPE_H_
