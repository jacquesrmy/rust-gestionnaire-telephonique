.PHONY: all fmt check clippy test ci

all: ci

fmt:
	cargo fmt --all

check:
	cargo check --all-targets --all-features

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-targets --all-features

ci:
	cargo fmt --all -- --check
	cargo check --all-targets --all-features
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-targets --all-features

clean:
	cargo clean