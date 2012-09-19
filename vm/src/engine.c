#include <stdlib.h>

#include "gi.h"

int engine_step(Engine_t *, char *);
static void engine_instruction_return(Engine_t *, char *);
static void engine_instruction_noop(Engine_t *);
static void engine_instruction_not_supported(Engine_t *);

void engine_create(Engine_t **engine) {
  *engine = malloc(sizeof(**engine));
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
    default: engine_instruction_not_supported(engine); return 0;
  };

  return 1;
}

int engine_return_value(Engine_t *engine) {
  return engine->return_value;
};

static void engine_instruction_return(Engine_t *engine, char *arguments) {
  printf(":return %d\n", (int)(*arguments) & 0xff);
  engine->return_value = *arguments;
}

static void engine_instruction_noop(Engine_t *engine) {
  printf(":noop\n");
}

static void engine_instruction_not_supported(Engine_t *engine) {
  printf(":instruction not supported\n");
}
