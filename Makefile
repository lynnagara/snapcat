.PHONY: build

build:
	cargo build --release
	./target/release/snaps
	open output.png

install:
	pip install -e .