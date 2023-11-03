all: client_example target/debug/client_example

client_example: target/debug/client_example
	cp target/debug/client_example client_example

target/debug/client_example: src/main.rs
	cargo build

.PHONY: clean
clean:
	cargo clean
	rm client_example
