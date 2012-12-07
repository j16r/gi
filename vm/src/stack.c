#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "stack.h"

#define DEFAULT_SIZE 1024

int stack_create(Stack_t **stack) {
  assert(stack);

  *stack = malloc(sizeof(**stack));
  if(*stack == NULL) {
    return 1;
  }

  /*(*stack)->size = DEFAULT_SIZE;*/
  (*stack)->buffer = calloc(DEFAULT_SIZE, sizeof(ptr_t));
  (*stack)->top = (*stack)->buffer + DEFAULT_SIZE;

  return 0;
}

void stack_destroy(Stack_t *stack) {
  free(stack->buffer);
  free(stack);
}

void stack_push(Stack_t *stack, ptr_t value) {
  stack->top -= 1;
  *stack->top = value;
}

ptr_t stack_pop(Stack_t *stack) {
  ptr_t value = *stack->top;
  stack->top += 1;
  return value;
}
