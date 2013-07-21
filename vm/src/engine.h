#ifndef _ENGINE_H_
#define _ENGINE_H_

typedef struct {
  int return_value;
  char *current_instruction;
  SymbolTable_t *symbols;
  Stack_t *stack;
  SymbolMap_t *values;
} Engine_t;

enum ENGINE_RETURN {
  ENG_CONTINUE,
  ENG_INVALID,
  ENG_RETURN
};

typedef struct {
  int value;
} Expression_t;

int engine_step(Engine_t *, char *);

#endif
