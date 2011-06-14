#include "includes.H"

giArgumentList &giArgumentList::add(
    const std::string & name,
    giClassPtr& value) {

  _values.add(name, value);
}

giClass::giClassPtr giArgumentList::value(size_t index) {
  return _values.lookup(index);
}

giClass::giClassPtr giArgumentList::value(const std::string & name) {
  return _values.lookup(name);
}
