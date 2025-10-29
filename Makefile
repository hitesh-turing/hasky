.PHONY: help build test run clean docker-build docker-run docker-test install fmt lint check

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in release mode
	cargo build --release

build-debug: ## Build the project in debug mode
	cargo build

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with output
	cargo test -- --nocapture

run: ## Run the application (use ARGS="your args" to pass arguments)
	cargo run -- $(ARGS)

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

install: ## Install the binary locally
	cargo install --path .

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

lint: ## Run clippy linter
	cargo clippy -- -D warnings

check: fmt-check lint test ## Run all checks (format, lint, test)

docker-build: ## Build Docker image
	docker build -t hashy:latest .

docker-run: ## Run Docker container with help
	docker run --rm hashy:latest --help

docker-test: ## Run tests in Docker
	docker run --rm hashy:latest hash --text "test"

docker-shell: ## Start a shell in the Docker container
	docker run --rm -it --entrypoint /bin/bash hashy:latest

# Development workflow targets
dev: fmt lint test ## Run development checks

watch: ## Watch for changes and run tests
	cargo watch -x test

watch-run: ## Watch for changes and run the app
	cargo watch -x 'run -- hash --help'

# Release targets
release: clean check build ## Prepare a release build

# Documentation
doc: ## Generate and open documentation
	cargo doc --open

doc-private: ## Generate documentation including private items
	cargo doc --document-private-items --open

