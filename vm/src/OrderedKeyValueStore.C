#include "includes.H"

template<class T>
giOrderedKeyValueStore<T> &giOrderedKeyValueStore<T>::add(const std::string & name, T & value) {

  _list.push_back(value);
  _map[name] = value;

}

template<class T>
T giOrderedKeyValueStore<T>::lookup(size_t index) {
  return _list[index];
}

template<class T>
T giOrderedKeyValueStore<T>::lookup(const std::string & name) {
  return _map[name];
}
