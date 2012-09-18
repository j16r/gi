#include <stdlib.h>
#include <stdio.h>
#include <check.h>

#include "../src/symbol_table.h"

SymbolTable_t *symbols = NULL;

static void setup(void) {
  symbol_table_create(&symbols);
}

static void teardown(void) {
  symbol_table_dump(symbols, stdout);
  symbol_table_destroy(symbols);
}

START_TEST(test_symbol_table_create) {

  fail_if(symbols == NULL);
  fail_unless(symbols->count == 0);
  fail_unless(symbols->size > 0);
  fail_if(symbols->symbols == NULL);

} END_TEST

START_TEST(test_symbol_table_add) {

  Symbol_t ident;
  fail_if(symbol_table_add(symbols, L"symbol", &ident));
  fail_unless(symbols->count == 1);
  fail_unless(symbols->counts[0] == 1);
  fail_if(symbol_table_add(symbols, L"new symbol", &ident));
  fail_unless(symbols->count == 2);
  fail_unless(symbols->counts[1] == 1);
  fail_if(symbol_table_add(symbols, L"symbol", &ident));
  fail_unless(symbols->count == 2);
  fail_unless(symbols->counts[0] == 2);

} END_TEST

START_TEST(test_symbol_table_add_utf8) {

  size_t ident;
  fail_if(symbol_table_add(symbols, L"Schöne Grüße", &ident));
  fail_unless(symbols->count == 1);

} END_TEST

START_TEST(test_symbol_table_lookup) {

  size_t ident;
  fail_if(symbol_table_add(symbols, L"new symbol", &ident));
  wchar_t *symbol = NULL;
  fail_if(symbol_table_lookup(symbols, ident, &symbol));
  fail_if(wcscmp(L"new symbol", symbol));

} END_TEST

Suite *make_symbol_table_test_suite(void) {
  Suite *suite = suite_create("symbol_table");

  /* Core test case */
  TCase *tc_core = tcase_create("symbol table core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_symbol_table_create);
  tcase_add_test(tc_core, test_symbol_table_add);
  tcase_add_test(tc_core, test_symbol_table_add_utf8);
  tcase_add_test(tc_core, test_symbol_table_lookup);
  suite_add_tcase(suite, tc_core);

  return suite;
}
