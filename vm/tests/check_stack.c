#include <check.h>

#include "../src/gi.h"
#include "../src/stack.h"

Stack_t *stack = NULL;

static void setup(void) {
  stack_create(&stack);
}

static void teardown(void) {
  stack_destroy(stack);
}

START_TEST(test_stack_create) {

  fail_if(stack == NULL);
  fail_if(stack_empty(stack) == 0);

} END_TEST

START_TEST(test_stack_push) {

  stack_push(stack, (ptr_t)0x666);
  fail_unless(*stack->top == (ptr_t)0x666);
  fail_unless(stack_empty(stack) == 0);

} END_TEST

START_TEST(test_stack_pop) {

  stack_push(stack, (ptr_t)0xbadbeef);
  fail_unless(stack_pop(stack) == (ptr_t)0xbadbeef);
  fail_if(stack_empty(stack) == 0);

} END_TEST

Suite *make_stack_test_suite(void) {
  Suite *suite = suite_create("stack");

  TCase *tc_core = tcase_create("stack core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_stack_create);
  tcase_add_test(tc_core, test_stack_push);
  tcase_add_test(tc_core, test_stack_pop);
  suite_add_tcase(suite, tc_core);

  return suite;
}
