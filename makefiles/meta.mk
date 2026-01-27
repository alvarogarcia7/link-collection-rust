# Meta targets for project setup and verification

PREK_INSTALL_DIR ?= $(HOME)/.local/bin
PREK := $(PREK_INSTALL_DIR)/prek

$(PREK): ## Install Prek locally if not already installed
	cargo install --locked --root $(HOME)/.local prek
.PHONY: $(PREK)

install-prek: $(PREK) ## Install Prek locally in ~/.local/bin
	@echo "Prek installed at $(PREK)"
.PHONY: install-prek

install-githooks: $(PREK) ## Install git hooks with Prek
	$(PREK) install
.PHONY: install-githooks

init: \
	install-prek \
	install-githooks \
	format-check \
	test ## Initialize this project in this folder (or git worktree) after clone
.PHONY: init

# Verification targets for essential goals
verify-essential-targets: \
	verify-build \
	verify-init \
	verify-test \
	verify-format \
	verify-lint ## Verify all essential targets exist
.PHONY: verify-essential-targets

verify-build: ## Verify build target exists
	@${MAKE} -n build > /dev/null 2>&1 || (echo "ERROR: build target not found" && exit 1)
	@echo "✓ build target verified"
.PHONY: verify-build

verify-init: ## Verify init target exists
	@${MAKE} -n init > /dev/null 2>&1 || (echo "ERROR: init target not found" && exit 1)
	@echo "✓ init target verified"
.PHONY: verify-init

verify-test: ## Verify test target exists
	@${MAKE} -n test > /dev/null 2>&1 || (echo "ERROR: test target not found" && exit 1)
	@echo "✓ test target verified"
.PHONY: verify-test

verify-format: ## Verify format target exists
	@${MAKE} -n format > /dev/null 2>&1 || (echo "ERROR: format target not found" && exit 1)
	@echo "✓ format target verified"
.PHONY: verify-format

verify-lint: ## Verify clippy (lint) target exists
	@${MAKE} -n clippy > /dev/null 2>&1 || (echo "ERROR: clippy (lint) target not found" && exit 1)
	@echo "✓ clippy (lint) target verified"
.PHONY: verify-lint
