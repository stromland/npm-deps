.PHONY: install
install: fmt lint
	cargo build --release
	cp ./target/release/npm-deps ~/.cargo/bin

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy