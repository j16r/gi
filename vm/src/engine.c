#include <stdlib.h>

#include "gi.h"

void engine_create(Engine_t **engine) {
  *engine = malloc(sizeof(**engine));
}

void engine_destroy(Engine_t *engine) {
  free(engine);
}

void engine_run(Engine_t * engine, char * program) {
}

void engine_step(Engine_t * engine, char * instruction) {
}
