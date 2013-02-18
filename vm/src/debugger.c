#include "gi.h"
#include "stack.h"
#include "symbol_table.h"
#include "symbol_map.h"
#include "engine.h"
#include "debugger.h"

void unhandled_exception(Engine_t *engine) {
  printf("Unhandled exception!\n");
  printf("Current instruction: %d\n", *engine->current_instruction);
  symbol_table_dump(engine->symbols, stdout);
  exit(1);
}
