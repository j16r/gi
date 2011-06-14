#ifndef _ARGUMENT_LIST_H_
#define _ARGUMENT_LIST_H_

#include "includes.H"

class giClass;

class giArgumentList : giClass {

  giOrderedKeyValueStore<giClass> _values;

public:

  giArgumentList& add(const std::string &, giClassPtr);

  giClassPtr value(size_t);
  giClassPtr value(const std::string &);

  size_t count() const { return _values.count(); }
};

#endif // _ARGUMENT_LIST_H_
