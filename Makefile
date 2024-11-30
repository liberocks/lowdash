# Makefile

# Variables
CARGO = cargo


# Targets
.PHONY: all test build-dev build-release format coverage

all: test build-dev build-release format coverage

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