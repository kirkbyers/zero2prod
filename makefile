watch:
	cargo watch -x "run --bin zero2prod"

lint:
	cargo clippy

fmt:
	cargo fmt

lint-ci:
	cargo clippy -- -D warnings

test:
	cargo test