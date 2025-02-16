format:
	cargo fmt --all --
.PHONY: format

format-check:
	cargo fmt --all -- --check
.PHONY: format-check

test: up
	cargo test --all --all-features --tests
.PHONY: test

test-expensive:
	cargo test -- --ignored
.PHONY: test-expensive

test-all: test test-expensive test-cli
.PHONY: test-all

test-cli: test-cli-ls

test-cli-ls:
	cargo run --bin select -- ls ./data_access/data/links.rec
.PHONY: test-cli-ls

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy

doc:
	cargo doc --all-features
.PHONY: doc

all: format clippy test doc

up:
	docker compose up -d

down:
	docker compose down

build:
	cargo build
.PHONY: build

build-release:
	cargo build --release
.PHONY: build-release

LC:=cargo run --release --bin lc --
PRO:=--environment pro

import: build-release
	@if [ -z "$(hn)" ]; then echo "`hn` is not set. use make ... hn=..."; exit 1; fi
	$(LC) $(PRO) new-record import:$(hn)
.PHONY: import

add: build-release
	$(LC) $(PRO) new-record
.PHONY: add

sync:
	make -C ~/Documents/project/link-collection sync
.PHONY: sync
