#include "gi.h"
#include "symbol_table.h"
#include "engine.h"
#include "engine_instructions.h"

int engine_step(Engine_t *, char *);

void engine_create(Engine_t **engine) {
  *engine = malloc(sizeof(**engine));
  symbol_table_create(&(*engine)->symbols);
  (*engine)->return_value = 0;
}

void engine_destroy(Engine_t *engine) {
  free(engine);
}

void engine_run(Engine_t *engine, char *program) {
  for(engine->current_instruction = program;
      engine_step(engine, engine->current_instruction);
      ++engine->current_instruction)
    ;
}

int engine_step(Engine_t *engine, char *program) {
  char instruction  = *program;
  char *arguments = program + 1;

  switch(instruction) {
    case 0: engine_instruction_noop(engine); break;
    case 1: engine_instruction_return(engine, arguments); return 0;

    case 2: engine_instruction_define_function(engine, arguments); break;

    default: engine_instruction_not_supported(engine); return 0;
  };

  return 1;
}

int engine_return_value(Engine_t *engine) {
  return engine->return_value;
};
