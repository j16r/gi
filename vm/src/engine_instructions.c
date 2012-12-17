#include <string.h>

#include "gi.h"
#include "stack.h"
#include "symbol_table.h"
#include "symbol_map.h"
#include "engine.h"

void engine_instruction_noop(Engine_t *engine) {
  printf(":noop\n");
  engine->current_instruction += 1;
}

void engine_instruction_return(Engine_t *engine, char *arguments) {
  printf(":return %d\n", (int)(*arguments) & 0xff);
  engine->return_value = *arguments;
  engine->current_instruction = stack_pop(engine->stack);
}

void engine_instruction_not_supported(Engine_t *engine) {
  printf(":instruction not supported\n");
}

void engine_instruction_define_function(Engine_t *engine, char *arguments) {
  printf(":define function\n");
  Symbol_t symbol;
  const char * function_name = arguments;
  symbol_table_add(engine->symbols, function_name, &symbol);
  symbol_map_add(engine->values, symbol, (void *)&engine->current_instruction);

  engine->current_instruction += 1 + strlen(function_name) + 1;
}

void engine_instruction_call_function(Engine_t *engine, char *arguments) {
  printf(":call function\n");
  char *function_name = arguments;
  Symbol_t symbol;
  char *function_address;

  symbol_table_add(engine->symbols, function_name, &symbol);
  symbol_map_fetch(engine->values, symbol, (void *)&function_address);

  char *return_address = engine->current_instruction + 1 + strlen(function_name) + 1;
  stack_push(engine->stack, return_address);
  engine->current_instruction = function_address;
}

void engine_instruction_load_module(Engine_t * engine, char *arguments) {
  printf(":load module\n");
}
