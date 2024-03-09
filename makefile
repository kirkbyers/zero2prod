watch:
	cargo watch -x check -x test -x run

lint:
	cargo clippy

fmt:
	cargo fmt

lint-ci:
	cargo clippy -- -D warnings

scan-ci:
	cargo 