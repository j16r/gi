#include "includes.H"

giFunction::giFunction() : giClass(GI_FUNCTION, __FILE__) {
}

giClass::giClassPtr giFunction::instance(
    Visibility visibility,
    giArgumentSpecification &specification) {

  giClassPtr new_instance(create_instance());

  boost::dynamic_pointer_cast<giFunction>(new_instance)->_visibility = visibility;
  boost::dynamic_pointer_cast<giFunction>(new_instance)->_specification = specification;

  return new_instance;
}

giClass::giClassPtr giFunction::invoke(
    giClassPtr scope,
    giClassPtr sender,
    ArgumentList &arguments) {

}

//giClass::giClassPtr giClass::invoke(const std::string & method_name, giClass::ArgumentList & args) {
  //std::cout << "Invoking " << method_name << " with " << args.size() << " arguments" << std::endl;
  //giMethod func = _methods[method_name];
  //std::cout << "Function ptr " << func << std::endl;
  //return (this->*func)(args);
//}
