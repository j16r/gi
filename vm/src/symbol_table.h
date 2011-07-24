#ifndef _SYMBOL_TABLE_H_
#define _SYMBOL_TABLE_H_

#define DEFAULT_SIZE 1024

typedef struct {
  size_t count;
  size_t size;
  size_t *counts;
  wchar_t **symbols;
} SymbolTable_t;

int symbol_table_create(SymbolTable_t **);
int symbol_table_add(SymbolTable_t *, const wchar_t *, size_t *);
int symbol_table_lookup(SymbolTable_t *, size_t, wchar_t **);

void symbol_table_dump(SymbolTable_t *, FILE *);

#endif
