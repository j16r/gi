#include <check.h>

#include "../src/gi.h"
#include "../src/symbol_table.h"
#include "../src/symbol_map.h"

SymbolMap_t *map = NULL;

static void setup(void) {
  symbol_map_create(&map);
}

static void teardown(void) {
  symbol_map_destroy(map);
}

START_TEST(test_symbol_map_create) {

  fail_if(map == NULL);

} END_TEST

START_TEST(test_symbol_map_add) {

  char * value1 = "deadgirl";
  char * value2 = "aliveboy";
  char * retrieved = NULL;

  symbol_map_add(map, 0, value1);
  symbol_map_add(map, 1, value2);
  fail_unless(0 == symbol_map_fetch(map, 0, &retrieved));
  fail_unless(retrieved == value1);
  fail_unless(0 == symbol_map_fetch(map, 1, &retrieved));
  fail_unless(retrieved == value2);
  /*fail_unless(0 != symbol_map_fetch(map, 2, &retrieved));*/

} END_TEST

START_TEST(test_symbol_map_fetch) {
} END_TEST

Suite *make_symbol_map_test_suite(void) {
  Suite *suite = suite_create("map");

  TCase *tc_core = tcase_create("map core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_symbol_map_create);
  tcase_add_test(tc_core, test_symbol_map_add);
  tcase_add_test(tc_core, test_symbol_map_fetch);
  suite_add_tcase(suite, tc_core);

  return suite;
}
