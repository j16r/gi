#ifndef _OBJECT_H_
#define _OBJECT_H_

class giObject {

  typedef boost::shared_ptr<giObject> ObjectPtr;

public:

  // Method invocation
  ObjectPtr invoke(const std::string & method_name);

};

#endif // _OBJECT_H_
