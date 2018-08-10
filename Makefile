.PHONY: build install

build:
	pushd ./rust; cargo build --release; popd;
	./rust/target/release/snapcat ./images/pikachu1.png ./images/pikachu2.png 0.2
	open output.png

install:
	pip install -e .