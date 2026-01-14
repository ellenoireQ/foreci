.PHONY: all build-rust build-go clean run

all: build-rust build-go

build-rust:
	cargo build --release

build-go:
	cd runner && go build -o ../bin/runner

clean:
	cargo clean
	rm -f bin/runner

run: all
	./target/release/foreci