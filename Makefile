all: check doc build test
build:
	cargo build --all
build-release:
	cargo build --all --release
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
.PHONY: all build build-release check doc test

ci:
	docker build -t remexre/stahlnet-rs-builder .travis
	docker run -v "$(shell pwd):/code" --rm remexre/stahlnet-rs-builder make doc test build-release
	docker run -v "$(shell pwd):/code" --rm remexre/stahlnet-rs-builder make ci-fix-perms
ci-fix-perms:
	chown $(shell stat -c '%u:%g' Makefile) -R target
.PHONY: ci ci-fix-perms
