.PHONY: strictdoc-install
.PHONY: strictdoc-validate
.PHONY: strictdoc-build
.PHONY: strictdoc-clean
.PHONY: strictdoc-view
.PHONY: strictdoc

STRICTDOC_VERSION ?= latest
STRICTDOC_EXPORT_DIR ?= build/strictdoc
STRICTDOC_REQUIREMENTS_DIR ?= .

# Check if strictdoc is installed
STRICTDOC := $(shell command -v strictdoc 2>/dev/null)

strictdoc-install: ## Install StrictDoc (requires Python 3.9+)
	@if command -v strictdoc >/dev/null 2>&1; then \
		echo "✓ StrictDoc is already installed"; \
		strictdoc version; \
	else \
		echo "Installing StrictDoc..."; \
		pip install strictdoc; \
	fi

strictdoc-validate: ## Validate StrictDoc requirements syntax
	@if [ -z "$(STRICTDOC)" ]; then \
		echo "❌ StrictDoc not installed. Run 'make strictdoc-install'"; \
		exit 1; \
	fi
	@echo "Validating StrictDoc requirements..."
	@mkdir -p /tmp/strictdoc-validate
	@strictdoc export $(STRICTDOC_REQUIREMENTS_DIR) \
		--output-dir /tmp/strictdoc-validate \
		&& echo "✓ All requirements are valid" \
		|| (echo "❌ Requirements validation failed"; exit 1)
	@rm -rf /tmp/strictdoc-validate

strictdoc-build: ## Build StrictDoc HTML documentation
	@if [ -z "$(STRICTDOC)" ]; then \
		echo "❌ StrictDoc not installed. Run 'make strictdoc-install'"; \
		exit 1; \
	fi
	@echo "Building StrictDoc documentation..."
	@mkdir -p $(STRICTDOC_EXPORT_DIR)
	@strictdoc export $(STRICTDOC_REQUIREMENTS_DIR) \
		--output-dir $(STRICTDOC_EXPORT_DIR) \
		&& echo "✓ Documentation built to $(STRICTDOC_EXPORT_DIR)" \
		|| (echo "❌ Build failed"; exit 1)

strictdoc-view: ## Open StrictDoc documentation in browser (macOS/Linux)
	@if [ ! -d "$(STRICTDOC_EXPORT_DIR)" ]; then \
		echo "Documentation not built. Run 'make strictdoc-build' first"; \
		exit 1; \
	fi
	@if command -v open >/dev/null 2>&1; then \
		open $(STRICTDOC_EXPORT_DIR)/index.html; \
	elif command -v xdg-open >/dev/null 2>&1; then \
		xdg-open $(STRICTDOC_EXPORT_DIR)/index.html; \
	else \
		echo "Open $(STRICTDOC_EXPORT_DIR)/index.html in your browser"; \
	fi

strictdoc-clean: ## Clean StrictDoc build artifacts
	@echo "Cleaning StrictDoc artifacts..."
	@rm -rf $(STRICTDOC_EXPORT_DIR)
	@echo "✓ Cleaned"

strictdoc: strictdoc-validate strictdoc-build ## Validate and build StrictDoc documentation