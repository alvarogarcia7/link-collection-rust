# Targets for application-specific operations

LC:=cargo run --release --bin lc --
PRO:=--environment pro

import: build-release ## Import links by hostname (use: make import hn=<hostname>)
	@if [ -z "$(hn)" ]; then echo "$(hn) is not set. use make import hn=..."; exit 1; fi
	$(LC) $(PRO) new-record import:$(hn)
.PHONY: import

add: build-release ## Add a new link record interactively
	$(LC) $(PRO) new-record
.PHONY: add

sync: ## Sync with external link collection project
	make -C ~/Documents/project/link-collection sync
.PHONY: sync
