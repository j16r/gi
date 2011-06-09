#ifndef _ORDERED_KEY_VALUE_STORE_H_
#define _ORDERED_KEY_VALUE_STORE_H_

template<class T>
class giOrderedKeyValueStore {

  std::map<std::string, T> _map;
  std::vector<T> _list;

public:

  giOrderedKeyValueStore &add(const std::string &, T &);

  T lookup(size_t);
  T lookup(const std::string &);
};

#endif // _ORDERED_KEY_VALUE_STORE_H_
