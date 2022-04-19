.PHONY: install
install: fmt lint
	cargo build --release
	rm ~/.cargo/bin/npm-deps &> /dev/null || true
	cp ./target/release/npm-deps ~/.cargo/bin

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy