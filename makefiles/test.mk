# Targets for testing

start-stubs: ## Start HTTP stub servers for testing
	cargo build --release --package stubs
	./target/release/stubs -p 8181 ./downloader/tests/http-stubs &
.PHONY: start-stubs

stop-stubs: ## Stop HTTP stub servers
	pkill -f "stubs -p 8181" || true
.PHONY: stop-stubs

test: start-stubs ## Run unit and integration tests
	cargo test --all --all-features --tests
	-${MAKE} stop-stubs
.PHONY: test

test-expensive: ## Run expensive (ignored) tests
	cargo test -- --ignored
.PHONY: test-expensive

test-cli: test-cli-ls ## Run CLI tests
.PHONY: test-cli

test-cli-ls: ## Test CLI command: list links
	cargo run --bin select -- ls ./data_access/data/links.rec
.PHONY: test-cli-ls

test-all: test test-expensive test-cli ## Run all tests (standard, expensive, and CLI)
.PHONY: test-all
