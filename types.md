# Types

Types are a function that get evaluated against a name.

Nominative types:

func do_something(x: X) {
}

X = func(argumentType) {
  argument.Type == X
}

Structural types:

func do_something(x: #frobnicate) {
}

X = func(argument) {
  argument.has_method?(frobnicate)
}

func do_something_else(x: #foo,#baz) {
}

Hierarchical subtyping:

type X implements Y

func do_something(y: Y) {
}

Y = fun

Affine types:

func do_something() {
  var x : X = 10;
  // <- end of x
}
