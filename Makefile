
help:  ## Display this help
		@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

start: ## Start Server
	spacetime start

build: ## Build Server and Client
	cd ./server && \
		spacetime publish -c spacetime-bevy-game
	cargo run --mainfest-path ./client/Cargo.toml

generate-server-bindings: ## Generate Spacetime Reducer Bindings
	mkdir client/src/module_bindings
	spacetime generate --lang rust \
		--out-dir client/src/module_bindings \
		--project-path server


