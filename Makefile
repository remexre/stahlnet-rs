all: check doc build test
build:
	cargo build --all
check:
	cargo check --all
doc:
	cargo doc --all
	cp doc-index.html target/doc/index.html
test:
	cargo test --all -- --test-threads=1
watch:
	cargo watch -s $(MAKE)
.PHONY: all build check doc test
