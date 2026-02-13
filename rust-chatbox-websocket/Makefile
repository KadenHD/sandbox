APP_NAME := rust-chatroom-websocket

.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make dev             - Run the application with auto-reload (dev config)"
	@echo "  make stg             - Run the application in staging environment"
	@echo "  make prod            - Run the application in production environment"
	@echo "  make build           - Build debug binary"
	@echo "  make release         - Build release binary"
	@echo "  make test            - Run tests"
	@echo "  make check           - Check code without building"
	@echo "  make fmt             - Format code"
	@echo "  make lint            - Run clippy"
	@echo "  make clean           - Clean target directory"

.PHONY: dev
dev:
	cargo watch -x run

.PHONY: stg
stg: release
	./target/release/$(APP_NAME)

.PHONY: prod
prod: release
	./target/release/$(APP_NAME)

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: check
check:
	cargo check

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy -- -D warnings

.PHONY: clean
clean:
	cargo clean
