#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "symbol_table.h"

#define DEFAULT_SIZE 1024

int symbol_table_create(SymbolTable_t **table) {
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

void symbol_table_destroy(SymbolTable_t *table) {
  assert(table);

  Symbol_t index;
  for(index = 0; index < table->count; ++index) {
    free(table->symbols[index]);
  }
  free(table->symbols);
  free(table);
}

int symbol_table_add(SymbolTable_t *table, const char *identifier, Symbol_t *symbol) {
  assert(table);
  assert(identifier);
  assert(symbol);

  Symbol_t index;
  for(index = 0; index < table->count; ++index) {
    if(strcmp(identifier, table->symbols[index]) == 0) {
      table->counts[index] += 1;
      *symbol = index;
      return 0;
    }
  }
  /*expand the symbol table as we have run out of space */
  if(index == table->size) {
    return 1;
  }

  table->symbols[index] = (char *)malloc((strlen(identifier) + 1) *sizeof(char));
  if(table->symbols[index] == NULL) {
    return 1;
  }

  strcpy(table->symbols[index], identifier);
  table->counts[index] += 1;
  table->count += 1;
  *symbol = index;
  return 0;
}

int symbol_table_lookup(SymbolTable_t *table, Symbol_t symbol, char **identifier) {
  assert(table);
  assert(identifier);

  if(symbol < table->count && table->symbols[symbol]) {
    *identifier = table->symbols[symbol];
    return 0;
  }
  return 1;
}

void symbol_table_dump(SymbolTable_t *table, FILE *output) {
  assert(table);
  assert(output);

  fprintf(output, "Symbol table, size: %lu, items: %lu\n", table->size, table->count);
  Symbol_t index;
  for(index = 0; index < table->count; ++index) {
    fprintf(output, "Item[%lu] (%lu) => %s\n", index, table->counts[index], table->symbols[index]);
  }
  fprintf(output, "\n");
}
