ifneq (,$(wildcard ./.env))
    include .env
    export
endif

build:
	cargo build

run: build
	RUST_LOG=debug cargo run
