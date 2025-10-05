.PHONY: help build build-release check run test test-verbose test-coverage bench lint fmt fmt-check doc clean update install-tools watch ci pre-commit
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[36m
RESET := \033[0m

# build - Build the project in debug mode
build:
	@cargo build

# check - Quick compile check without building
check:
	@cargo check --all-targets --all-features

# run - Run the project
run:
	@cargo run

# test - Run all tests
test:
	@cargo test

# test-verbose - Run tests with detailed output
test-verbose:
	@cargo test -- --nocapture --test-threads=1

# test-coverage - Generate test coverage report (requires cargo-tarpaulin)
test-coverage:
	@cargo tarpaulin --out Html --output-dir coverage

# bench - Run benchmarks (requires nightly)
bench:
	@cargo +nightly bench

# lint - Run clippy for linting
lint:
	@cargo clippy --all-targets --all-features -- -D warnings

# fmt - Format code
fmt:
	@cargo fmt --all

# fmt-check - Check code formatting without modifying files
fmt-check:
	@cargo fmt --all -- --check

# doc - Build and open documentation
doc:
	@cargo doc --no-deps --open

# doc-all - Build documentation with dependencies
doc-all:
	@cargo doc --open

# clean - Remove build artifacts
clean:
	@cargo clean

# update - Update dependencies
update:
	@cargo update

# install-tools - Install required development tools
install-tools:
	@rustup component add clippy rustfmt
	@rustup toolchain install nightly
	@cargo install cargo-watch cargo-tarpaulin cargo-audit cargo-outdated 2>/dev/null || true

# watch - Watch for changes and run check + test
watch:
	@cargo watch -x check -x test -x run

# audit - Check for security vulnerabilities
audit:
	@cargo audit

# outdated - Check for outdated dependencies
outdated:
	@cargo outdated

# ci - Run all CI checks (lint, test, build)
ci: fmt-check lint test build
	@echo "$(BLUE)✓ All CI checks passed!$(RESET)"

# pre-commit - Run before committing (format, lint, test)
pre-commit: fmt lint test
	@echo "$(BLUE)✓ Pre-commit checks passed!$(RESET)"

# help - Show this help message
help:
	@echo ""
	@echo "Usage:"
	@echo "    make [target]"
	@echo ""
	@echo "Targets:"
	@awk '/^[a-zA-Z\-_0-9]+:/ \
	{ \
		helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "$(BLUE)%-22s$(RESET) %s\n", helpCommand, helpMessage; \
		} \
	} { lastLine = $$0 }' $(MAKEFILE_LIST)
	@echo ""
