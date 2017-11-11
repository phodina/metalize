GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

all: build

clean:
	$(CARGO_BIN) clean
	rm -f ./bin/*
build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -fsanitize=address -o ./bin/addition ./examples/addition.c -L. -l:target/release/libmetalize.a