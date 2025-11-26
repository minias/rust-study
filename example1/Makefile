# ----------------------------------------
# Rust Project Makefile
# ----------------------------------------

APP_NAME=exam3
TARGET_DIR=target/release
INSTALL_PATH=/usr/local/bin

# ----------------------------------------
# Help
# ----------------------------------------
help:
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  build       Build release binary"
	@echo "  run         Run app with arguments (ARGS=\"...\")"
	@echo "  debug       Build debug version"
	@echo "  test        Run tests"
	@echo "  clean       Remove target directory"
	@echo "  install     Install binary to /usr/local/bin"
	@echo ""
	@echo "Examples:"
	@echo "  make run ARGS=\"4 9 8 2 6 3\""
	@echo ""

# ----------------------------------------
# Build targets
# ----------------------------------------
build:
	cargo build --release

debug:
	cargo build

run: build
	$(TARGET_DIR)/$(APP_NAME) $(ARGS)

test:
	cargo test

clean:
	cargo clean

# ----------------------------------------
# Install binary globally
# ----------------------------------------
install: build
	install -m 755 $(TARGET_DIR)/$(APP_NAME) $(INSTALL_PATH)

