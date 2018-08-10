.PHONY: build install

build:
	pushd ./rust; cargo build; popd;
	./rust/target/debug/snapcat ./images/cat.png ./images/cat-too-dark.png 0.2
	open output.png

install:
	pip install -e .