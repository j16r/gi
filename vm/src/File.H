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

  giClass::giClassPtr instance(ArgumentList & args);

  void constructor(ArgumentList & args);
  giClass::giClassPtr read(ArgumentList & args);
  giClass::giClassPtr write(ArgumentList & args);

};

#endif // _FILE_H_
