build: pre
	cargo build --all-targets
	cargo doc

pre:
	cargo deny check licenses
	cargo fmt --all -- --check
	cargo clippy --all --all-targets

publish_py: test_py
	docker pull quay.io/pypa/manylinux2014_x86_64
	docker run -it --rm -v $(shell pwd):/xxh3 quay.io/pypa/manylinux2014_x86_64 /xxh3/py_publish.sh

test_py: install_py
	python3 -m unittest discover

install_py: pre
	maturin develop

test: pre
	RUST_BACKTRACE=1 cargo test
