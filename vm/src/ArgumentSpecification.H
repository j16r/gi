#ifndef _ARGUMENT_SPECIFICATION_H_
#define _ARGUMENT_SPECIFICATION_H_

class giClass;

class giArgumentSpecification : giClass {

  struct ArgumentHolder_t {
    giClassPtr _default_value;
    giClassPtr _class_constraint;

    ArgumentHolder_t(giClassPtr default_value, giClassPtr class_constraint) :
      _default_value(default_value),
      _class_constraint(class_constraint) {
    }
  };

  typedef boost::shared_ptr<ArgumentHolder_t> Argument;

  giOrderedKeyValueStore<Argument> _values;

public:

  giArgumentSpecification();
  ~giArgumentSpecification();

  giArgumentSpecification &add(const std::string &, giClassPtr, giClassPtr);

  giClassPtr default_value(size_t);
  giClassPtr default_value(const std::string &);

  giClassPtr class_constraint(size_t);
  giClassPtr class_constraint(const std::string &);
};

#endif // _ARGUMENT_SPECIFICATION_H_
