GCC = gcc
INC = -I"/usr/local/include"
LIB = -dead_strip -L"/usr/local/apr/lib" -lapr-1
CFLAGS = -g -O0 -DDEBUG -DDYLD_LIBRARY_PATH

# VM Targets
VM_SRC = $(wildcard src/*.c)
VM_OBJS = $(VM_SRC:.c=.o)

bin/%.o : src/%.c
	$(GCC) $(CFLAGS) $(INC) -c -o $@ $<

bin/gi: $(VM_OBJS)
	$(GCC) $(CFLAGS) $(LIB) $(INC) -o $@ $^

# Test targets
TEST_SRC=$(VM_SRC) $(wildcard tests/*.c)
TEST_OBJS=$(TEST_SRC:.c=.o)

bin/%.o : tests/%.c
	$(GCC) $(CFLAGS) $(INC) -c -o $@ $<

bin/test: $(TEST_OBJS)
	$(GCC) $(CFLAGS) $(LIB) -lcheck $(INC) -o $@ $^

# Dummy targets
all: bin/gi bin/test

clean:
	rm bin/*
