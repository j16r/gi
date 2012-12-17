#ifndef _SYMBOL_MAP_H_
#define _SYMBOL_MAP_H_

#include <stddef.h>
#include <stdio.h>

typedef struct {
  size_t count;
  size_t size;
  void **values;
} SymbolMap_t;

int symbol_map_create(SymbolMap_t **);
void symbol_map_destroy(SymbolMap_t *);
void symbol_map_add(SymbolMap_t *, Symbol_t, void *);
int symbol_map_fetch(SymbolMap_t *, Symbol_t, void **);

void symbol_map_dump(SymbolMap_t *, FILE *);

#endif
