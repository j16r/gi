#ifndef _FUNCTION_H_
#define _FUNCTION_H_

class giFunction : public giClass {

  Visibility _visibility;
  giArgumentSpecification _specification;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giFunction());
  }

public:

  giFunction();

  typedef giClassPtr (giFunction::*giMethod)(ArgumentList & args);

  giClassPtr instance(Visibility, giArgumentSpecification &);

  giClassPtr invoke(giClassPtr, giClassPtr, ArgumentList &);

};

#endif // _FUNCTION_H_
