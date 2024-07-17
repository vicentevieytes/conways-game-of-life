# Define variables
CARGO = cargo
FMT = $(CARGO) fmt --all
CLIPPY = $(CARGO) clippy --all-targets --all-features -- -D warnings
BUILD = $(CARGO) build --release

# Default target
all: fmt clippy build

# Format code
fmt:
	$(FMT)

# Run Clippy linter
clippy:
	$(CLIPPY)

# Build project
build:
	$(BUILD)

# Clean target
clean:
	$(CARGO) clean

# Test target
test:
	$(CARGO) test

# Run target (assuming main.rs is your entry point)
run:
	$(CARGO) run

# Help message
help:
	@echo "Available targets:"
	@echo "  all      - Run fmt, clippy, and build"
	@echo "  fmt      - Format code"
	@echo "  clippy   - Run Clippy linter"
	@echo "  build    - Build project"
	@echo "  clean    - Clean build artifacts"
	@echo "  test     - Run tests"
	@echo "  run      - Run the project"
	@echo "  help     - Show this help message"

.PHONY: all fmt clippy build clean test run help
