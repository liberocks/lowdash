# Makefile

# Variables
CARGO = cargo


# Targets
.PHONY: all test build-dev build-release format coverage clippy bench check

all: test build-dev build-release format coverage clippy bench check

test:
	$(CARGO) test

build-dev:
	$(CARGO) build

build-release:
	$(CARGO) build --release

format:
	$(CARGO) fmt

coverage:
	cargo-tarpaulin --engine llvm --follow-exec --post-test-delay 10 --exclude-files benches/*

clippy:
	$(CARGO) clippy

bench:
	$(CARGO) bench

check: format test coverage clippy
