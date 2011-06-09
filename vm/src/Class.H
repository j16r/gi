#ifndef _CLASS_H_
#define _CLASS_H_

class giClass {

  const std::string _name;
  const std::string _file_name;

  void process(const char *, const std::streamsize&);

public:

  typedef enum { Private, Protected, Public } Visibility;
  typedef boost::shared_ptr<giClass> giClassPtr;
  typedef giArgumentList<giClassPtr> ArgumentList;
  typedef std::map<std::string, giClassPtr> Slot;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giClass());
  }

public:

  Slot _slots;

  giClass();
  giClass(const std::string&, const std::string&);
  giClass(const std::string&, const std::string&, fs::ifstream &);
  inline virtual ~giClass() {};

  inline const std::string& name() const { return _name; }

  giClassPtr instance(giClassPtr, ArgumentList &);

  giClassPtr lookup(const std::string &);
};

#endif // _CLASS_H_
