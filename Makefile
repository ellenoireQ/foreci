.PHONY: all build-rust build-go clean run install uninstall

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

all: build-rust build-go

build-rust:
	cargo build --release

build-go:
	cd runner && go build -o ../bin/runner

clean:
	cargo clean
	rm -f bin/runner

run: all
	./target/release/easydocker

install: all
	@echo "Installing easydocker to $(BINDIR)..."
	sudo mkdir -p $(BINDIR)
	sudo cp target/release/easydocker $(BINDIR)/easydocker
	sudo cp bin/runner $(BINDIR)/easydocker-runner
	sudo chmod +x $(BINDIR)/easydocker
	sudo chmod +x $(BINDIR)/easydocker-runner
	@echo "Installation complete! Run 'easydocker' to start."

uninstall:
	@echo "Uninstalling easydocker from $(BINDIR)..."
	sudo rm -f $(BINDIR)/easydocker
	sudo rm -f $(BINDIR)/easydocker-runner
	@echo "Uninstallation complete."