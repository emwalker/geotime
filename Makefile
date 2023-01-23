check: format test
	cargo check

check-pre-push: check

format:
	cargo clippy --fix -- -D warnings -D clippy::panic -D clippy::panic_in_result_fn -D clippy::panicking_unwrap
	cargo fmt

test:
	cargo test
