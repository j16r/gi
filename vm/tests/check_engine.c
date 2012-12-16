#include <check.h>

#include "../src/gi.h"
#include "../src/symbol_table.h"
#include "../src/stack.h"
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
  fail_if(engine->symbols == NULL);
  fail_if(engine->stack == NULL);

} END_TEST

START_TEST(test_engine_run_noop) {

  char bytecode[] = {0, 1, 0};
  engine_run(engine, bytecode);

} END_TEST

START_TEST(test_engine_return) {

  char return_value = 0x7f;
  char bytecode[] = {1, return_value};
  engine_run(engine, bytecode);
  fail_unless(engine_return_value(engine) == return_value);

} END_TEST

START_TEST(test_engine_define_function) {

  char bytecode[] = {2, 'd', 'e', 'f', 0, 1, 0};
  engine_run(engine, bytecode);
  fail_unless(engine->symbols->count == 1);

} END_TEST

START_TEST(test_engine_call_function) {

  char bytecode[] = {2, 'd', 'e', 'f', 3, 'd', 'e', 'f', 1, 0};
  engine_run(engine, bytecode);

} END_TEST

/*START_TEST(test_engine_load_module) {*/

  /*char bytecode[] = {3, };*/
  /*engine_run(engine, bytecode);*/
  /*fail_unless();*/

/*} END_TEST*/

Suite *make_engine_test_suite(void) {
  Suite *suite = suite_create("engine");

  TCase *tc_core = tcase_create("engine core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_engine_create);
  tcase_add_test(tc_core, test_engine_run_noop);
  tcase_add_test(tc_core, test_engine_return);
  tcase_add_test(tc_core, test_engine_define_function);
  suite_add_tcase(suite, tc_core);

  return suite;
}
