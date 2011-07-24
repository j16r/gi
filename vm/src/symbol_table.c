#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

#include "symbol_table.h"

int symbol_table_create(SymbolTable_t ** table) {
  assert(table);

  *table = malloc(sizeof(**table));
  if(*table == NULL) {
    return 1;
  }

  (*table)->count = 0;
  (*table)->size = DEFAULT_SIZE;

  (*table)->counts = calloc((*table)->size, sizeof(*(*table)->counts));
  if((*table)->counts == NULL) {
    return 1;
  }

  (*table)->symbols = calloc((*table)->size, sizeof(**(*table)->symbols));
  if((*table)->symbols == NULL) {
    return 1;
  }

  return 0;
}

int symbol_table_destroy(SymbolTable_t * table) {
  assert(table);

  size_t index;
  for(index = 0; index < table->count; index++) {
    free(table->symbols[index]);
  }
  free(table->symbols);
  free(table);

  return 0;
}

int symbol_table_add(SymbolTable_t * table, const wchar_t * symbol, size_t * identifier) {
  assert(table);
  assert(identifier);

  size_t index;
  for(index = 0; index < table->count; index++) {
    if(wcscmp(symbol, table->symbols[index]) == 0) {
      table->counts[index] += 1;
      *identifier = index;
      return 0;
    }
  }
  /* expand the symbol table as we have run out of space */
  if(index == table->size) {
    return 1;
  }

  table->symbols[index] = (wchar_t *)malloc((wcslen(symbol) + 1) * sizeof(wchar_t));
  if(table->symbols[index] == NULL) {
    return 1;
  }

  wcscpy(table->symbols[index], symbol);
  table->counts[index] += 1;
  table->count += 1;
  *identifier = index;
  return 0;
}

int symbol_table_lookup(SymbolTable_t * table, size_t identifier, wchar_t ** symbol) {
  assert(table);
  assert(symbol);

  if(identifier < table->count && table->symbols[identifier]) {
    *symbol = table->symbols[identifier];
    return 0;
  }
  return 1;
}

void symbol_table_dump(SymbolTable_t * table, FILE * output) {
  assert(table);
  assert(output);

  fprintf(output, "Table size: %lu, items: %lu\n", table->size, table->count);
  size_t index;
  for(index = 0; index < table->count; index++) {
    fprintf(output, "Item[%lu] (%lu) => %ls\n", index, table->counts[index], table->symbols[index]);
  }
}
