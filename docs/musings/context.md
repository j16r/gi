# Musings on Scoping and Bindings

The ultimate goal here is to have some rules for scope and where definitions
are allocated and stored.

## Modules

The first scope rule I'd like here is the module. Modules are created
implicitly by a file, i.e. rsa.gi implicitly creates a namespace rsa. If a
function is defined in rsa, e.g: encrypt. Then it should be accessible at
rsa.encrypt.

Namespaces can be nested, directories create that kind of nesting. So
crypto/rsa will create a namespace crypto within that namespace will be a rsa
namespace. I'd like that same syntax to be usable in the language.

This implies that a name resolution procedure would begin from its current
module, and work its way up to the root namespace (/) to find a specific
definition.

  crypto/
    rsa/
      encrypt

How do we represent this in the most efficient way for dynamic lookup?

Typical usage:

  encrypt.gi:

    import crypto/rsa

    encrypt("hello")

Psuedo code:

  module = "encrypt"
  definitions["encrypt"] = module["crypto/rsa"].definitions

Explicit usage:

  crypto/rsa/encrypt("hello")

## Regular Function scope

In regular functions, the user defines a set of arguments to the function. e.g:

  func MyFunction(x String)

x Is defined and can be used from within the function body of MyFunction. For
gi I'm going to say that an argument in a function definition is final and
cannot be 'overridden' or 'shadowed'.

This is more of a dynamic scope, rather than a static scope. In that as we're
parsing a source file we might keep information about where we are in the
scope. So that we can lookup where in the source a definition was specified.

But a function scope, is declared at the function call site. e.g:

  MyFunction(10)

i.e.

  scope = newScope(top of the stack)
  scope.x = 10

In a statically compiled language, one might generate code like:

  define x = 10
  call MyFunction

Which might get reduced to

  mov %eax, 10
  push %eax
  jmp $MyFunction

In a more dynamic language we might say:

  binding = Binding::new()
  binging.insert("x", 10)
  scopes.push(binding)

## Closures

Closures are a special case(?) in that they should close over their local
scope. e.g:

  var x, y;
  func fooby() {
    print(x)
  }
  x = 10;
  fooby();

This should print 10.

crypto/rsa.encrypt:doperform
