#include <stdlib.h>
#include <check.h>

#include "../src/engine.h"

Engine_t *engine = NULL;

static void setup(void) {
  engine_create(&engine);
}

static void teardown(void) {
  engine_destroy(engine);
}

START_TEST(test_engine_create) {

  fail_if(engine == NULL);

} END_TEST

START_TEST(test_engine_run_noop) {

  char bytecode = 0;
  engine_run(engine, &bytecode);

} END_TEST

Suite *make_engine_test_suite(void) {
  Suite *suite = suite_create("engine");

  /* Core test case */
  TCase *tc_core = tcase_create("engine core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_engine_create);
  tcase_add_test(tc_core, test_engine_run_noop);
  suite_add_tcase(suite, tc_core);

  return suite;
}
