# Git Hooks Configuration (Prek)

This project uses [Prek](https://prek.j178.dev/) for managing git hooks. The configuration is defined in `.pre-commit-config.yaml`.

## Pre-Commit Stage

Hooks that run before each commit to catch issues early:

### cargo-format
- **Name:** Cargo format check
- **Command:** `cargo fmt --all -- --check`
- **Purpose:** Verify that Rust code adheres to the project's formatting standards
- **Execution:** Runs on all `.rs` files
- **Fail Behavior:** Fails if code formatting is incorrect (use `cargo fmt` to auto-fix)

## Pre-Push Stage

Hooks that run before pushing to remote to ensure code quality:

### cargo-clippy
- **Name:** Cargo clippy
- **Command:** `cargo clippy --all --all-features --tests -- -D warnings`
- **Purpose:** Run the Clippy linter to catch common mistakes and improve code quality
- **Execution:** Runs on all `.rs` files
- **Fail Behavior:** Fails if any clippy warnings are found (treated as errors with `-D warnings`)

### cargo-test
- **Name:** Cargo test
- **Command:** `make start-stubs && cargo test --all --all-features --tests; make stop-stubs`
- **Purpose:** Execute unit and integration tests with stubbed HTTP servers
- **Execution:** Runs on all `.rs` files
- **Fail Behavior:** Fails if any tests fail
- **Note:** Automatically starts and stops HTTP stub servers required for tests

### cargo-doc
- **Name:** Cargo doc
- **Command:** `cargo doc --all-features`
- **Purpose:** Generate documentation and verify all doc items are properly documented
- **Execution:** Runs on all `.rs` files
- **Fail Behavior:** Fails if there are documentation errors

## Usage

Install the hooks:
```bash
prek install
```

Run hooks manually:
```bash
# Run all pre-commit hooks
prek run --stage pre-commit

# Run all pre-push hooks
prek run --stage pre-push

# Run a specific hook
prek run --hook cargo-format
```

Bypass hooks (not recommended):
```bash
# Skip pre-commit hooks
git commit --no-verify

# Skip pre-push hooks (depends on git configuration)
git push --no-verify
```

## Hook Dependencies

These hooks correspond to the dependencies in `make all`:

- `format` → `cargo-format` (pre-commit stage)
- `clippy` → `cargo-clippy` (pre-push stage)
- `test` → `cargo-test` (pre-push stage)
- `doc` → `cargo-doc` (pre-push stage)

This staged approach allows for quick feedback on formatting issues before commit while keeping more comprehensive checks for the pre-push stage.
