#include <gi.h>
#include <symbol_table.h>
#include <symbol_map.h>
#include <stack.h>
#include <engine.h>

#include "functions.h"

int gi_lib_core_logic_if(Engine_t * engine) {
  Expression_t statement;

  gi_lib_core_arguments_fetch(engine, &statement);
  if(test_true(statement) == 0) {
    Expression_t true_expression;
    gi_lib_core_arguments_fetch(engine, &true_expression);
    evaluate_expression(true_expression);
  } else {
    Expression_t false_expression;
    gi_lib_core_arguments_fetch(engine, &false_expression);
    evaluate_expression(false_expression);
  }
}

int gi_lib_core_logic_and(Engine_t * engine) {
}

int gi_lib_core_logic_or(Engine_t * engine) {
}

int gi_lib_core_logic_not(Engine_t * engine) {
}
