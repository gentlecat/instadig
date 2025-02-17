ifneq (,$(wildcard ./.env))
    include .env
    export
endif

build:
	cargo build

run:
	RUST_LOG=INFO cargo run
