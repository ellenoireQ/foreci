.PHONY: all build-rust build-go clean run install uninstall

ifeq ($(OS),Windows_NT)
    EXT        = .exe
    RM         = cmd /C del /F /Q
    RMDIR      = cmd /C rmdir /S /Q
    MKDIR      = cmd /C mkdir
    CP         = cmd /C copy /Y
    RUN_PREFIX =
    SUDO       =
    INSTALL_DIR = $(USERPROFILE)\.local\bin
    define INSTALL_BIN
	powershell -Command "if (!(Test-Path '$(INSTALL_DIR)')) { New-Item -ItemType Directory -Path '$(INSTALL_DIR)' | Out-Null }"
	$(CP) "target\release\easydocker.exe" "$(INSTALL_DIR)\easydocker.exe"
	$(CP) "bin\easydocker-runner.exe" "$(INSTALL_DIR)\easydocker-runner.exe"
    endef
    define UNINSTALL_BIN
	$(RM) "$(INSTALL_DIR)\easydocker.exe" 2>nul
	$(RM) "$(INSTALL_DIR)\easydocker-runner.exe" 2>nul
    endef
else
    EXT        =
    RM         = rm -f
    RMDIR      = rm -rf
    MKDIR      = mkdir -p
    CP         = cp
    RUN_PREFIX = ./
    SUDO       = sudo
    PREFIX    ?= /usr/local
    INSTALL_DIR = $(PREFIX)/bin
    define INSTALL_BIN
	$(SUDO) $(MKDIR) $(INSTALL_DIR)
	$(SUDO) $(CP) target/release/easydocker $(INSTALL_DIR)/easydocker
	$(SUDO) $(CP) bin/easydocker-runner $(INSTALL_DIR)/easydocker-runner
	$(SUDO) chmod +x $(INSTALL_DIR)/easydocker
	$(SUDO) chmod +x $(INSTALL_DIR)/easydocker-runner
    endef
    define UNINSTALL_BIN
	$(SUDO) $(RM) $(INSTALL_DIR)/easydocker
	$(SUDO) $(RM) $(INSTALL_DIR)/easydocker-runner
    endef
endif

all: build-rust build-go

build-rust:
	cargo build --release

build-go:
	cd runner && go build -o ../bin/easydocker-runner$(EXT) .

clean:
	cargo clean
ifeq ($(OS),Windows_NT)
	-$(RM) "bin\easydocker-runner.exe" 2>nul
else
	$(RM) bin/easydocker-runner
endif

run: all
	$(RUN_PREFIX)target/release/easydocker$(EXT)

install: all
	@echo "Installing easydocker to $(INSTALL_DIR)..."
	$(INSTALL_BIN)
	@echo "Installation complete! Run 'easydocker' to start."

uninstall:
	@echo "Uninstalling easydocker from $(INSTALL_DIR)..."
	$(UNINSTALL_BIN)
	@echo "Uninstallation complete."