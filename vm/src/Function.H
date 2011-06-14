#ifndef _FUNCTION_H_
#define _FUNCTION_H_

class giFunction : public giClass {

  Visibility _visibility;
  giArgumentSpecification _specification;
  giMethod _method;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giFunction());
  }

public:

  giFunction();

  giClassPtr instance(Visibility, giArgumentSpecification &, giMethod);

  giClassPtr invoke(giClassPtr, giClassPtr, giArgumentList &);

};

#endif // _FUNCTION_H_
