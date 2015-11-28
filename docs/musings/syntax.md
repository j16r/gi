# Syntax

This document is mostly random mind dumps on ideas of syntax.

## Goals

  * Syntax is designed using a parser generator and is therefore sound
  * Syntax is somewhat familiar to ALGOL family users
  * Whitespace significant like python?
  * No semicolons
  * Use prefixes so that different syntax features are easily identifiable and parseable


## Functions

    fn name(argument1)

    fn name(argument1: Bool) String
        body

    fn name(argument2: 1 @ int) String

    fn name(argument2: int) String

Type restriction, i.e. `argument2: int` says that this function accepts at
least an int, and no less. Omitting the restriction means anything can be
placed into the function as long as the operations performed within are
compatible.

Functions can be overloaded provided the types are different.
Return types can be overloaded.

## Type declaration

    type x Int

    type x struct
      a
      b : Int32

    type x union
      a
      b : In32

## Variable

  
