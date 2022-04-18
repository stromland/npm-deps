.PHONY: install
install: fmt lint
	cargo build --release
	rm ~/.local/bin/npm-deps
	cp ./target/release/npm-deps ~/.local/bin

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy