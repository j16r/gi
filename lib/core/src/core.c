#include <gi.h>
#include <symbol_table.h>
#include <symbol_map.h>
#include <stack.h>
#include <engine.h>

#include "functions.h"

int gi_required(Engine_t * engine) {
  printf("Loaded core module!\n");

  gi_define_ext_function(engine, "if", &gi_lib_core_logic_if);
  gi_define_ext_function(engine, "and", &gi_lib_core_logic_and);
  gi_define_ext_function(engine, "or", &gi_lib_core_logic_or);
  gi_define_ext_function(engine, "not", &gi_lib_core_logic_not);
};
