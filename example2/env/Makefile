# ----------------------------------------
# Rust Project Makefile
# ----------------------------------------

APP_NAME=env
TARGET_DIR=target/release # Ext SSD 라서 오류가 발생
# 아래 메시지는 외장 SSD 라서 나며, 빌드는 정상
# warning: hard linking files in the incremental compilation cache failed. copying files instead. ...
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

fix:
	cargo fix --lib -p env --allow-dirty

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

