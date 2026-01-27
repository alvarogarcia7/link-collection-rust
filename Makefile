# Include modular makefiles organized by responsibility
include makefiles/rust.mk
include makefiles/clippy.mk
include makefiles/test.mk
include makefiles/app.mk

all: format clippy test doc ## Run all quality checks (format, clippy, test, doc)
.PHONY: all

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
.PHONY: help
