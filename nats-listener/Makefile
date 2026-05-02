.PHONY: build test fmt lint clean run help

help:
	@echo "HackerNews NATS Listener - Build Commands"
	@echo ""
	@echo "build          - Build the listener"
	@echo "build-release  - Build release binary"
	@echo "test           - Run all tests"
	@echo "test-unit      - Run unit tests only"
	@echo "test-integration - Run integration tests only"
	@echo "test-verbose   - Run tests with output"
	@echo "run            - Run the listener (requires NATS running)"
	@echo "fmt            - Format code"
	@echo "fmt-check      - Check code formatting"
	@echo "lint           - Run clippy linter"
	@echo "clean          - Clean build artifacts"
	@echo "check          - Check code without building"
	@echo ""

build:
	cargo build -p nats-listener

build-release:
	cargo build --release -p nats-listener

test:
	cargo test -p nats-listener

test-unit:
	cargo test -p nats-listener --lib

test-integration:
	cargo test -p nats-listener --test integration_tests

test-verbose:
	RUST_LOG=debug cargo test -p nats-listener -- --nocapture

run:
	cargo run -p nats-listener

run-release: build-release
	./target/release/nats-listener

fmt:
	cargo fmt -p nats-listener

fmt-check:
	cargo fmt -p nats-listener -- --check

lint:
	cargo clippy -p nats-listener -- -D warnings

check:
	cargo check -p nats-listener

clean:
	cargo clean
	rm -f Cargo.lock

docs:
	cargo doc -p nats-listener --open

all: fmt lint test build
	@echo "✓ All checks passed"
