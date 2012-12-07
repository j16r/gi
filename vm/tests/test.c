#include <stdlib.h>
#include <check.h>

Suite *make_engine_test_suite(void);
Suite *make_symbol_table_test_suite(void);
Suite *make_stack_test_suite(void);

int main(int argc, char *argv[]) {
  int number_failed;

  Suite *suite = make_symbol_table_test_suite();

  SRunner *suite_runner = srunner_create(suite);
  srunner_add_suite(suite_runner, make_engine_test_suite());
  srunner_add_suite(suite_runner, make_stack_test_suite());

  srunner_run_all(suite_runner, CK_VERBOSE);
  number_failed = srunner_ntests_failed(suite_runner);
  srunner_free(suite_runner);

  return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
