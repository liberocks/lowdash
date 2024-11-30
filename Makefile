# Makefile

# Variables
CARGO = cargo


# Targets
.PHONY: all test build-dev build-release format coverage clippy

all: test build-dev build-release format coverage clippy

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

check: format test coverage clippy
