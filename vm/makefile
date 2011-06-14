GCC = gcc
INC = -I"/usr/local/include"
LIB = -dead_strip
CFLAGS = -g -O0 -DDEBUG

bin/%.o : src/%.c
	$(GCC) $(CFLAGS) $(INC) -c -o $@ $<

bin/gi: bin/main.o
	$(GCC) $(CFLAGS) $(LIB) $(INC) -o $@ $^

all: bin/gi

clean:
	rm bin/*
