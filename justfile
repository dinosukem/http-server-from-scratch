fmt:
    cargo +nightly fmt --all

lint:
    cargo +nightly clippy --all-targets --all-features -- -D warnings -D clippy::enum_glob_use

ci: fmt lint

run:
	cargo run --release http-from-scratch
