build:
	cargo build
	gcc -o target/main ./src_c/main.c -I./src -L ./target/debug -lffi_lib
	./target/main