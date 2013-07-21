#include <gi.h>
#include <symbol_table.h>
#include <symbol_map.h>
#include <stack.h>
#include <engine.h>

#include "functions.h"

uint8_t test_true(Expression_t statement) {
  return statement.value;
}

uint8_t test_false(Expression_t statement) {
  return !test_false(statement);
}

// Generates a true expression
int gi_lib_core_bool_true(Engine_t * engine) {
}

// Generates a false expression
int gi_lib_core_bool_false(Engine_t * engine) {
}

// Tests if an expression is true
int gi_lib_core_bool_test_true(Engine_t * engine) {
}

// Tests if an expression is false
int gi_lib_core_bool_test_false(Engine_t * engine) {
}
