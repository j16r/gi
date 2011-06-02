GCC = g++
INC = -I"/usr/local/include"
LIB = -dead_strip \
	-lstdc++ \
  -lboost_system-mt \
  -lboost_filesystem-mt \
  -lboost_program_options-mt \
  -lboost_thread-mt
CFLAGS = -g -O0 -DDEBUG

bin/%.o : src/%.C
	$(GCC) $(CFLAGS) $(INC) -c -o $@ $<

bin/gi: bin/main.o bin/Array.o bin/Class.o bin/Engine.o bin/File.o bin/Exception.o bin/String.o bin/Integer.o
	$(GCC) $(CFLAGS) $(LIB) $(INC) -o $@ $^

all: bin/gi

clean:
	rm bin/*
