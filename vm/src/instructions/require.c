#include <string.h>

#include "../gi.h"
#include "../stack.h"
#include "../symbol_table.h"
#include "../symbol_map.h"
#include "../engine.h"
#include "../debugger.h"
#include "../engine_instructions.h"

#include <dlfcn.h>

void engine_instruction_require(Engine_t *engine, char *arguments) {
  printf(":require ");
  char *module_name = arguments;
  size_t module_name_length = strlen(module_name) + 1;

  printf("%s\n", module_name);
  void *module = dlopen(module_name, RTLD_LAZY | RTLD_LOCAL);
  if(module) {
    engine->current_instruction += 1 + module_name_length;
  } else {
    engine_instruction_raise_exception(engine, NULL);
  }
}
