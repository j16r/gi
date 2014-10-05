# Data Definition Language

The idea here is to be able to define data that can be stored and manipulated by the gi language.

I want to start with EBNF or similar and the primitive building blocks of bits. All data theoretically can be represented with some sequence of 0s and 1s.

Most programming languages allow you to define something like a string which can hold any arbitrary sequence of characters. I would like to be able to express data with more restrictions where helpful.

In a typical OO based programming language a URL might be represented by a class which stores url components in separate string variables. If you were to write something like:

URL.create("https//invalid")

An obviously invalid url, you'd have to wait till runtime to find out (this example code would throw an exception in Java).

Now if the user were to provide a url at runtime, handling the error at runtime would be the only thing we could do. But what if we could ensure that data objects were only constructed in ways they were allowed at a language level.

(Would this even be a good idea, let's find out?)

zero = 0
one = 1
bit = 0 | 1
byte = { 8 * bit }
word = { 2 * byte }
int = { 2 * word }
long = { 2 * int }

Now we come to a problem, say we want to represent a signed byte. Well this information is not a unique property of the data itself, but the way we treat it.

What if we did:

signed bit = 0 | 1
signed byte = signed bit, { 7 * bit }
signed word = signed bit, { 15 * bit }
signed int = signed bit, { 31 * bit }
signed long = signed bit, { 63 * bit }

That 'signed bit' has the same physical representation, though because it's metadata i.e. its name is different, we can differentiate. This seems useful, but I imagine that storing this meta data alongside each instance would take a lot of space. (What if we stored everything in buckets? Individual bits would be much more compact this way).

How about a url? Let's start with the protocol. The protocol is any sequence of non whitespace characters followed by :// .

As regex:
scheme = [^\s]+://
EBNF:
whitespace = "\n" | "\r" | "\t" | " "
nonwhitespace = characterspace - whitespace
scheme = { nonwhitespace }, "://"

functions produce data
