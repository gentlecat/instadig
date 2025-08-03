ifneq (,$(wildcard ./.env))
    include .env
    export
endif

fmt:
	cargo fmt

build: fmt
	cargo build

run: build
	cargo run
