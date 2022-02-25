build:
	cargo clippy
	cargo fmt
	cargo build

test:
	cargo test

clean:
	cargo clean
