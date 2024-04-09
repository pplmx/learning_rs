.PHONY: help build run test bench fmt clean
.DEFAULT_GOAL := help

# build
build:
	@echo "Building..."
	@cargo build

# run
run:
	@echo "Running..."
	@cargo run

# test
test:
	@echo "Testing..."
	@cargo test

# bench
bench:
	@echo "Benchmarking..."
	@rustup default nightly
	@cargo bench
	@rustup default stable

# fmt
fmt:
	@echo "Formatting..."
	@cargo fmt

# clean
clean:
	@echo "Cleaning..."
	@cargo clean

# Show help
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
			printf "\033[36m%-22s\033[0m %s\n", helpCommand,helpMessage; \
		} \
	} { lastLine = $$0 }' $(MAKEFILE_LIST)
