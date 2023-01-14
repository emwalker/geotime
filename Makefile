check: format test
	cargo check

check-pre-push: check

format:
	cargo clippy -- -D warnings
	cargo fmt

test:
	cargo test
