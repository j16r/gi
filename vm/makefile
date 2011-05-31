GCC = g++
INC = -I"/usr/local/include"
LIB = -dead_strip \
	-lstdc++ \
  -lboost_system-mt \
  -lboost_filesystem-mt \
  -lboost_program_options-mt \
  -lboost_thread-mt

bin/%.o : src/%.C
	$(GCC) $(INC) -c -o $@ $<

bin/gi: bin/main.o bin/Array.o bin/Class.o bin/Engine.o bin/File.o
	$(GCC) $(LIB) $(INC) -o $@ $^

all: bin/gi

clean:
	rm bin/*
