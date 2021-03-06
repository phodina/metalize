GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

all: build

clean:
	$(CARGO_BIN) clean
	rm -f ./bin/*
build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -fsanitize=address -o ./bin/addition ./examples/addition.c -Iinclude -L. -l:target/release/libmetalize.a -ldl -lpthread
	$(GCC_BIN) -fsanitize=address -o ./bin/count_characters ./examples/count_characters.c -Iinclude -L. -l:target/release/libmetalize.a -ldl -lpthread
	$(GCC_BIN) -fsanitize=address -o ./bin/quotes ./examples/quotes.c -Iinclude -L. -l:target/release/libmetalize.a -ldl -lpthread
	$(GCC_BIN) -fsanitize=address -o ./bin/sum ./examples/sum.c -Iinclude -L. -l:target/release/libmetalize.a -ldl -lpthread
	$(GCC_BIN) -fsanitize=address -o ./bin/point ./examples/point.c -Iinclude -L. -l:target/release/libmetalize.a -ldl -lpthread
	$(GCC_BIN) -fsanitize=address -o ./bin/accounts ./examples/accounts.c -Iinclude -L. -l:target/release/libmetalize.a -ldl -lpthread