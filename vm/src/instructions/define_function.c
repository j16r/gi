#include <string.h>

#include "../gi.h"
#include "../stack.h"
#include "../symbol_table.h"
#include "../symbol_map.h"
#include "../engine.h"
#include "../debugger.h"
#include "../engine_instructions.h"

int gi_define_function(Engine_t * engine, const char * function_name, const char * function_entry_point) {
  Symbol_t symbol;
  symbol_table_add(engine->symbols, function_name, &symbol);
  symbol_map_add(engine->values, symbol, (void *)function_entry_point);
}

void engine_instruction_define_function(Engine_t *engine, char *arguments) {
  printf(":define function\n");
  const char *function_name = arguments;
  size_t method_name_length = strlen(function_name) + 1;

  const char *function_entry_point = engine->current_instruction + 1 +
    method_name_length + 1;
  gi_define_function(engine, function_name, function_entry_point);

  size_t next_instruction = 1 + method_name_length +
    *(arguments + method_name_length) + 1;
  engine->current_instruction += next_instruction;
}
