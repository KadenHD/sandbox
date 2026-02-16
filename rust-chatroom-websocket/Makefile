APP_NAME := rust-chatroom-websocket

.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make dev             - Run the application with auto-reload (dev config)"
	@echo "  make stage           - Run the application in staging environment"
	@echo "  make prod            - Run the application in production environment"
	@echo "  make build           - Build debug binary"
	@echo "  make release         - Build release binary"
	@echo "  make tests           - Run tests"
	@echo "  make coverage        - Show the code coverage of the tests"
	@echo "  make check           - Check code without building"
	@echo "  make fmt             - Format code"
	@echo "  make lint            - Run clippy"
	@echo "  make docker          - Create docker container for DB"
	@echo "  make clean           - Clean target directory"

.PHONY: dev
dev:
	APP_ENV=development cargo watch -x run -q

.PHONY: stage
stage: release
	APP_ENV=staging ./target/release/$(APP_NAME)

.PHONY: prod
prod: release
	APP_ENV=production ./target/release/$(APP_NAME)

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: tests
tests:
	cargo test --release -- --test-threads=1 --nocapture

.PHONY: coverage
coverage:
	cargo tarpaulin --exclude-files src/main.rs

.PHONY: check
check:
	cargo check

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy -- -D warnings

.PHONY: docker
docker:
	docker compose up -d

.PHONY: clean
clean:
	cargo clean
