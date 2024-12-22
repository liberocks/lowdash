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
	cargo-tarpaulin

clippy:
	$(CARGO) clippy

bench:
	$(CARGO) bench

check: format test coverage clippy
