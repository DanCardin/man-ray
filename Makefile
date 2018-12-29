.PHONY: clean build test lint format

clean:
	cargo clean

build:
	cargo build

test:
	cargo test --verbose

lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

format:
	cargo fmt --all
