# Meta targets for project setup and verification

install-githooks: ## Install git hooks with Prek
	prek install
.PHONY: install-githooks

init: ## Initialize this project in this folder (or git worktree) after clone
	${MAKE} install-githooks
	${MAKE} format-check
	${MAKE} test
.PHONY: init
