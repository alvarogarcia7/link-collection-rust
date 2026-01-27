# Targets for Rust build system

build: ## Build the project in debug mode
	cargo build
.PHONY: build

build-release: ## Build the project in release mode
	cargo build --release
.PHONY: build-release

format: ## Format Rust code with cargo fmt
	cargo fmt --all --
.PHONY: format

format-check: ## Check if Rust code is properly formatted
	cargo fmt --all -- --check
.PHONY: format-check

doc: ## Generate documentation for all crates
	cargo doc --all-features
.PHONY: doc
