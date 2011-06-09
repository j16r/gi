#ifndef _ARGUMENT_LIST_H_
#define _ARGUMENT_LIST_H_

template<class T>
class giArgumentList {

  giOrderedKeyValueStore<T> _values;

public:

  giArgumentList& add(const std::string &, T&);

  T &value(size_t);
  T &value(const std::string &);
};

#endif // _ARGUMENT_LIST_H_
