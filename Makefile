.PHONY: build install

build:
	cargo build --release
	./target/release/snapcat ./images/pikachu1.png ./images/pikachu2.png 0.2
	open output.png

install:
	pip install -e .