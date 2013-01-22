#include "gi.h"
#include "symbol_table.h"
#include "symbol_map.h"
#include "stack.h"
#include "engine.h"
#include "engine_instructions.h"

int engine_step(Engine_t *, char *);

void engine_create(Engine_t **engine) {
  *engine = malloc(sizeof(**engine));

  symbol_table_create(&(*engine)->symbols);
  stack_create(&(*engine)->stack);
  symbol_map_create(&(*engine)->values);

  (*engine)->return_value = 0;
}

void engine_destroy(Engine_t *engine) {
  symbol_map_destroy(engine->values);
  stack_destroy(engine->stack);
  symbol_table_destroy(engine->symbols);

  free(engine);
}

void engine_run(Engine_t *engine, char *program) {
  engine->current_instruction = program;
  while(engine_step(engine, engine->current_instruction));
}

int engine_step(Engine_t *engine, char *program) {
  char instruction  = *program;
  char *arguments = program + 1;

  switch(instruction) {
    case OC_NOOP:
      engine_instruction_noop(engine);
      break;
    case OC_RETURN:
      if(engine_instruction_return(engine, arguments)) {
        return 0;
      }
      break;
    case OC_DEF:
      engine_instruction_define_function(engine, arguments);
      break;
    case OC_CALL:
      engine_instruction_call_function(engine, arguments);
      break;
    default:
      engine_instruction_not_supported(engine);
      return 0;
  };

  return 1;
}

int engine_return_value(Engine_t *engine) {
  return engine->return_value;
};
