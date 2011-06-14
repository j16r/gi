#include "includes.H"

giArgumentSpecification::giArgumentSpecification() {
}

giArgumentSpecification::~giArgumentSpecification() {
}

giArgumentSpecification& giArgumentSpecification::add(
    const std::string & name,
    giClass::giClassPtr default_value,
    giClass::giClassPtr class_constraint) {

  Argument arg(new ArgumentHolder_t(default_value, class_constraint));
  _values.add(name, arg);
}

giClass::giClassPtr giArgumentSpecification::default_value(size_t index) {
  return _values.lookup(index)->_default_value;
}

giClass::giClassPtr giArgumentSpecification::default_value(const std::string & name) {
  return _values.lookup(name)->_default_value;
}

giClass::giClassPtr giArgumentSpecification::class_constraint(size_t index) {
  return _values.lookup(index)->_class_constraint;
}

giClass::giClassPtr giArgumentSpecification::class_constraint(const std::string & name) {
  return _values.lookup(name)->_class_constraint;
}

//void giArgumentSpecification::check_arguments(const giArgumentList & required_arguments, const giArgumentList & actual_arguments) {
  //for(giArgumentList::const_iterator required = required_arguments.begin();
      //required != required_arguments.end();
      //++required) {

    //giArgumentList::const_iterator actual = actual_arguments.find(required->first);

    //// Argument specified like (arg, ...
    //if(required->second == _c(GI_NIL)) {
      //if(actual == actual_arguments.end()) {
        //// no default was specified and no argument was specified
        //throw EXCEPTION->instance(_c(GI_CLASS), __FILE__, "");
      //}

    //// Argument specified like (arg: String, ...
    //} else if(actual != actual_arguments.end()) {
      //// TODO: needs to traverse up the tree
      //if(actual->second != required->second) {
        //// type mismatch exception

      //}

      

    //} else {

    //}

  //}
//}
