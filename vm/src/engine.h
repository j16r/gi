#ifndef _ENGINE_H_
#define _ENGINE_H_

typedef struct {
  int return_value;
  char *current_instruction;
  SymbolTable_t * symbols;
  Stack_t * stack;
} Engine_t;

#endif
