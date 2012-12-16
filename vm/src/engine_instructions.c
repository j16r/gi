#include "gi.h"
#include "stack.h"
#include "symbol_table.h"
#include "engine.h"

void engine_instruction_noop(Engine_t *engine) {
  printf(":noop\n");
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
  symbol_table_add(engine->symbols, arguments, &symbol);
}

void engine_instruction_call_function(Engine_t *engine, char *arguments) {
  printf(":call function\n");
  char *symbol = NULL;
  Symbol_t ident = (int)(*arguments) & 0xff;
  symbol_table_lookup(engine->symbols, ident, &symbol);
  stack_push(engine->stack, engine->current_instruction);
}

void engine_instruction_load_module(Engine_t * engine, char *arguments) {
  printf(":load module\n");
}
