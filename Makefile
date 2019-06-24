all: check doc build test
build:
	cargo build --all
build-dist:
	mkdir -p target/dist
	cd relay && cargo build --release --no-default-features
	cp target/release/stahlnet-relay target/dist/stahlnet-relay-lite
	cd relay && cargo build --release --no-default-features --features nogui
	cp target/release/stahlnet-relay target/dist/stahlnet-relay-nogui
	cd relay && cargo build --release
	cp target/release/stahlnet-relay target/dist/stahlnet-relay
check:
	cargo check --all
doc:
	cargo doc --all
	cp doc-index.html target/doc/index.html
test:
	cargo test --all -- --test-threads=1
	cargo test --all --no-default-features -- --test-threads=1
watch:
	cargo watch -s $(MAKE)
.PHONY: all build build-dist check doc test
