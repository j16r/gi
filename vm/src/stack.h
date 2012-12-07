#ifndef _STACK_H_
#define _STACK_H_

#include <stddef.h>
#include <stdio.h>

#define ptr_t char*

typedef struct {
  ptr_t *buffer;
  ptr_t *top;
} Stack_t;

int stack_create(Stack_t **);
void stack_push(Stack_t *stack, ptr_t value);
ptr_t stack_pop(Stack_t *stack);
int stack_top(Stack_t *, ptr_t*);

#endif
