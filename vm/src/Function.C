#include "includes.H"

giFunction::giFunction() : giClass(GI_FUNCTION, __FILE__) {
}

giClass::giClassPtr giFunction::instance(
    Visibility visibility,
    giArgumentSpecification &specification,
    giMethod method) {

  giClassPtr new_instance(create_instance());

  boost::dynamic_pointer_cast<giFunction>(new_instance)->_visibility = visibility;
  boost::dynamic_pointer_cast<giFunction>(new_instance)->_specification = specification;
  boost::dynamic_pointer_cast<giFunction>(new_instance)->_method = method;

  return new_instance;
}

giClass::giClassPtr giFunction::invoke(
    giClassPtr scope,
    giClassPtr sender,
    giArgumentList &arguments) {

  std::cout << "Invoking function on " << scope->name()
    << " from " << sender->name()
    << " with " << arguments.count() << " arguments" << std::endl;

  return (this->*_method)(arguments);
}
