#include "gi.h"
#include "symbol_table.h"
#include "engine.h"

void engine_instruction_noop(Engine_t *engine) {
  printf(":noop\n");
}

void engine_instruction_return(Engine_t *engine, char *arguments) {
  printf(":return %d\n", (int)(*arguments) & 0xff);
  engine->return_value = *arguments;
}

void engine_instruction_not_supported(Engine_t *engine) {
  printf(":instruction not supported\n");
}

void engine_instruction_define_function(Engine_t *engine, char *arguments) {
  printf(":define function\n");
  Symbol_t symbol;
  symbol_table_add(engine->symbols, arguments, &symbol);
}
