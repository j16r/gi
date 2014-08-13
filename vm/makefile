vm: **/*.rs
	rustc vm.rs

test: vm testrunner
	rustc --test vm.rs -o $@
	./$@

check: vm
	time ./vm run test.gi

all: vm test check
