.PHONY: build install

build:
	pushd ./rust; cargo build --release; popd;
	./rust/target/release/snapcat ./images/create.png ./images/create.png 0.2
	open output.png

install:
	pip install -e .