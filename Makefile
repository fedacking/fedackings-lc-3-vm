run:
	cargo run $(path)

build:
	cargo build

format:
	cargo fmt

format-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy

test:
	cargo test
