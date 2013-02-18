#include <check.h>

#include "../src/gi.h"
#include "../src/symbol_table.h"
#include "../src/symbol_map.h"
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
  fail_if(engine->values == NULL);

} END_TEST

START_TEST(test_engine_run_noop) {

  // 0 = opcode (noop)
  char bytecode[] = {OC_NOOP, 1, 0};
  engine_run(engine, bytecode);

} END_TEST

START_TEST(test_engine_return) {

  char return_value = 0x7f;
  // 1 = opcode (return)
  char bytecode[] = {OC_RETURN, return_value};
  engine_run(engine, bytecode);
  fail_unless(engine_return_value(engine) == return_value);

} END_TEST

START_TEST(test_engine_define_function) {

  // 2 = opcode (def)
  // def = function name
  // 0 = string null terminator
  // 1 = opcode (return)
  // 0 = return value
  char bytecode[] = {OC_DEF, 'd', 'e', 'f', 0, OC_RETURN, 0};
  engine_run(engine, bytecode);
  fail_unless(engine->symbols->count == 1);

} END_TEST

START_TEST(test_engine_call_function) {

  // 2 = opcode (def)
  // def = function name
  // 0 = string null terminator
  // 2 = bytes to skip
  // 1 = opcode (return)
  // 0 = return value
  // 3 = opcode (call)
  // def = function name
  // 0 = string null terminator
  // 1 = opcode (return)
  // 0 = return value
  char bytecode[] = {OC_DEF, 'd', 'e', 'f', 0, 3, 0, OC_RETURN, 0,
                     OC_CALL, 'd', 'e', 'f', 0,
                     OC_CALL, 'd', 'e', 'f', 0,
                     OC_CALL, 'd', 'e', 'f', 0,
                     OC_CALL, 'd', 'e', 'f', 0,
                     OC_CALL, 'd', 'e', 'f', 0,
                     OC_RETURN, 0};
  engine_run(engine, bytecode);
  fail_unless(engine->symbols->count == 1);
  fail_unless(*engine->current_instruction == OC_RETURN);

} END_TEST

START_TEST(test_engine_call_undefined_function) {

  char bytecode[] = {OC_CALL, 'w', 'a', 'z', 0,
                     OC_RETURN, 0};
  engine_run(engine, bytecode);
  fail_unless(engine->symbols->count == 0);
  fail_unless(*engine->current_instruction == OC_RETURN);

} END_TEST

START_TEST(test_engine_require) {

  char bytecode[] = {OC_REQUIRE, 'c', 'o', 'r', 'e', 0,
                     OC_RETURN, 0};
  engine_run(engine, bytecode);
  fail_unless(engine->symbols->count > 0);

} END_TEST

Suite *make_engine_test_suite(void) {
  Suite *suite = suite_create("engine");

  TCase *tc_core = tcase_create("engine core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_engine_create);
  tcase_add_test(tc_core, test_engine_run_noop);
  tcase_add_test(tc_core, test_engine_return);
  tcase_add_test(tc_core, test_engine_define_function);
  tcase_add_test(tc_core, test_engine_call_function);
  tcase_add_test(tc_core, test_engine_call_undefined_function);
  tcase_add_test(tc_core, test_engine_require);
  suite_add_tcase(suite, tc_core);

  return suite;
}
