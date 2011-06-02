#include <iostream>
#include <stdexcept>

#include <execinfo.h>

#include "Class.H"
#include "Array.H"
#include "Engine.H"
#include "File.H"
#include "Exception.H"

giEngine engine;

void handler() {
  void *trace_elems[128];
  int trace_elem_count(backtrace(trace_elems, 128));
  char **stack_syms(backtrace_symbols(trace_elems, trace_elem_count));
  for(int i = 0; i < trace_elem_count; ++i) {
      std::cout << stack_syms[i] << std::endl;
  }
  free(stack_syms);

  exit(1);
}

int main(int argc, char *argv[]) {
  //std::set_terminate(handler);

  engine.load_builtin_classes();
  engine.dump_classes();

  giClass::giArgumentList empty;

  try {
    engine.lookup_class("File")->invoke("read", empty);
  } catch(giClass::giClassPtr exception) {
    std::cout << "caught exception " << boost::dynamic_pointer_cast<giException>(exception)->dump();
  }
}
