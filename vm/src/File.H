#ifndef _FILE_H_
#define _FILE_H_

class giFile : public giClass {

  std::string _path;
  fs::fstream _file;

protected:

  giClassPtr create_instance() const {
    return giClassPtr(new giFile());
  }

public:

  giFile();

  giClass::giClassPtr instance(giArgumentList & args);

  void constructor(giArgumentList & args);
  giClass::giClassPtr read(giArgumentList & args);
  giClass::giClassPtr write(giArgumentList & args);

};

#endif // _FILE_H_
