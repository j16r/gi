#ifndef _FILE_H_
#define _FILE_H_

#include <vector>
#include <string>
#include <boost/filesystem.hpp>
#include <boost/filesystem/fstream.hpp>
#include <iostream>
#include <string>
#include <list>
namespace fs = boost::filesystem;

#include "Class.H"

class giFile : public giClass {

  fs::fstream _file;

public:

  giFile();

  giClass::ObjectPtr instance(giClass::giArgumentList & args);

  void constructor(giClass::giArgumentList & args);
  giClass::ObjectPtr read(giClass::giArgumentList & args);
  giClass::ObjectPtr write(giClass::giArgumentList & args);

};

#endif // _FILE_H_
