#include "includes.H"

#include <execinfo.h>

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

giClass::giClassPtr g_engine(new giEngine());

int main(int argc, char *argv[]) {
  std::set_terminate(handler);

  ENGINE->load_builtin_classes();
  ENGINE->dump_classes();

  giClass::ArgumentList empty;

  try {
    giClass *file_class(boost::dynamic_pointer_cast<giClass>(ENGINE->lookup_class(GI_FILE)).get());
    giFunction *function(boost::dynamic_pointer_cast<giFunction>(file_class->lookup("read")).get());
    function->invoke(ENGINE->lookup_class(GI_FILE), g_engine, empty);

  } catch(giClass::giClassPtr exception) {
    std::cout << "caught exception " << boost::dynamic_pointer_cast<giException>(exception)->dump();
    return -1;
  }

  return 0;
}
