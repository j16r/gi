#ifndef _NIL_H_
#define _NIL_H_

class giNil : public giClass {

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giNil());
  }

public:

  giNil() : giClass(GI_NIL, __FILE__) {};

  giClassPtr instance(ArgumentList & args) {
    std::cout << "Getting singleton Nil instance" << std::endl;
    return giClassPtr(this);
  };

  giClassPtr instance() {
    std::cout << "Getting singleton Nil instance" << std::endl;
    return giClassPtr(this);
  };
};

#endif // _FILE_H_

