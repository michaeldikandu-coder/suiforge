.PHONY: build test install clean fmt clippy doc help

help:
	@echo "SuiForge Build Commands:"
	@echo "  make build    - Build the project"
	@echo "  make test     - Run all tests"
	@echo "  make install  - Install SuiForge CLI"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make fmt      - Format code"
	@echo "  make clippy   - Run clippy linter"
	@echo "  make doc      - Generate documentation"

build:
	cargo build --release

test:
	cargo test --all-features

install:
	cargo install --path .

clean:
	cargo clean
	rm -rf target/

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

doc:
	cargo doc --no-deps --open

# Development commands
dev-build:
	cargo build

dev-test:
	cargo test

dev-run:
	cargo run -- help

# Release commands
release:
	cargo build --release
	strip target/release/suiforge

# Check everything before commit
check: fmt clippy test
	@echo "All checks passed!"
