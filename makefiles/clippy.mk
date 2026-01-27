# Targets for Clippy linter

clippy: ## Run Clippy linter with all features and treat warnings as errors
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy
