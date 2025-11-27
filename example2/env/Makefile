# ----------------------------------------
# Rust Project Makefile
# ----------------------------------------

APP_NAME := env
TARGET_DIR := target/release
INSTALL_PATH := /usr/local/bin

.PHONY: default
default: help

# ----------------------------------------
# Config Info (Variables)
# ----------------------------------------

# NOTE: 아래 정보는 Makefile 변수 기반으로 자동 생성됨.
config:
	@echo "# ----------------------------"
	@echo "# Makefile Config Info"
	@echo "# ----------------------------"
	@echo "# APP_NAME      = $(APP_NAME)"
	@echo "# TARGET_DIR    = $(TARGET_DIR)"
	@echo "# INSTALL_PATH  = $(INSTALL_PATH)"
	@echo "# ----------------------------"

# ----------------------------------------
# Help
# ----------------------------------------

#? help: Get more info on make commands.
help: Makefile
	@echo ''
	@echo 'blow the file required'
	@echo '	GNU Make v3.x / go v1.20.x '
	@echo ''
	@echo 'If you use a MacOs, you can install it as a brew. There is an option specifically for MacOs users.'
	@echo ''
	@echo 'Usage:'
	@echo '  make [target]'
	@echo ''
	@echo 'Targets:'
	@sed -n 's/^#?//p' $< | column -t -s ':' | sed -e 's/^/ /'
	@echo ''
	@echo '--- Makefile Variables ---'
	@echo " APP_NAME     = $(APP_NAME)"
	@echo " TARGET_DIR   = $(TARGET_DIR)"
	@echo " INSTALL_PATH = $(INSTALL_PATH)"
	@echo ''

#? tree: 프로젝트 소스파일 구조 확인명령어
tree:
	tree -I 'docs|._*|target|*.md|localhost-*'

#? build: 릴리즈 빌드 명령어
build:
	cargo build --release

#? debug: 디버그 빌드 명령어
debug:
	cargo build

#? fix: 사용하지 않는 패키지 수정 명령어
fix:
	cargo fix --lib -p ${APP_NAME} --allow-dirty

#? run: 빌드 파일 실행 (ARGS=인자)
run: build
	$(TARGET_DIR)/$(APP_NAME) $(ARGS)

#? test: cargo test
test:
	cargo test

#? clean: cargo clean
clean:
	cargo clean

#? install: 실행파일을 ${INSTALL_PATH} 에 복사
install: build
	install -m 755 $(TARGET_DIR)/$(APP_NAME) $(INSTALL_PATH)
