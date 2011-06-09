#include "includes.H"

template<class T>
giArgumentList<T> &giArgumentList<T>::add(
    const std::string & name,
    T &value) {

  _values.add(name, value);
}

template<class T>
T &giArgumentList<T>::value(size_t index) {
  return _values.lookup(index);
}

template<class T>
T &giArgumentList<T>::value(const std::string & name) {
  return _values.lookup(name);
}
