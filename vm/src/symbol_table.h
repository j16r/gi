#ifndef _SYMBOL_TABLE_H_
#define _SYMBOL_TABLE_H_

#include <stddef.h>
#include <stdio.h>

typedef size_t Symbol_t;

typedef struct {
  size_t count;
  size_t size;
  size_t *counts;
  char **symbols;
} SymbolTable_t;

int symbol_table_create(SymbolTable_t **);
int symbol_table_add(SymbolTable_t *, const char *, Symbol_t *);
int symbol_table_lookup(SymbolTable_t *, Symbol_t, char **);

void symbol_table_dump(SymbolTable_t *, FILE *);

#endif
