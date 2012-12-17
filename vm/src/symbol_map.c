#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "../src/symbol_table.h"
#include "symbol_map.h"

#define DEFAULT_SIZE 1024

int symbol_map_create(SymbolMap_t **map) {
  assert(map);

  *map = malloc(sizeof(**map));
  if(*map == NULL) {
    return 1;
  }

  (*map)->count = 0;
  (*map)->size = DEFAULT_SIZE;

  (*map)->values = calloc((*map)->size, sizeof(**(*map)->values));
  if((*map)->values == NULL) {
    return 1;
  }

  return 0;
}

void symbol_map_destroy(SymbolMap_t *map) {
  assert(map);

  free(map->values);
  free(map);
}

void symbol_map_add(SymbolMap_t *map, Symbol_t symbol, void *value) {
  assert(map);
  map->values[symbol] = value;
}

int symbol_map_fetch(SymbolMap_t *map, Symbol_t symbol, void **value) {
  assert(map);
  assert(value);

  *value = map->values[symbol];

  return 0;
}
