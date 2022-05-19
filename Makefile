build: pre
	cargo build --all-targets
	cargo doc

pre:
	cargo deny check licenses
	cargo fmt --all -- --check
	cargo clippy --all --all-targets

bench: pre
	cargo bench
	firefox ./target/criterion/report/index.html

flamegraph:
	cargo flamegraph --bench benchmark -- --bench
	firefox ./flamegraph.svg

publish_py: test_py
	docker pull quay.io/pypa/manylinux2014_x86_64
	docker run -it --rm -v $(shell pwd):/xxh3 quay.io/pypa/manylinux2014_x86_64 /xxh3/py_publish.sh

test_py: install_py
	python3 -m unittest discover

install_py: pre
	maturin develop

test: pre
	RUST_BACKTRACE=1 cargo test

# Nightly version selected from: https://rust-lang.github.io/rustup-components-history/
NIGHTLY := "nightly-2022-05-17"
fuzz: pre
	rustup toolchain install $(NIGHTLY)
	cargo +$(NIGHTLY) fuzz run fuzz_xxh3 -- -max_len=1000000

RUST_SYSROOT := $(shell cargo +$(NIGHTLY) rustc -- --print sysroot 2>/dev/null)
LLVM_COV := $(shell find $(RUST_SYSROOT) -name llvm-cov)
fuzz_coverage: pre
	echo $(LLVM_COV)
	rustup component add llvm-tools-preview --toolchain $(NIGHTLY)
	cargo +$(NIGHTLY) fuzz coverage fuzz_xxh3
	$(LLVM_COV) show fuzz/target/*/release/fuzz_xxh3 \
					--format html \
                    -instr-profile=fuzz/coverage/fuzz_xxh3/coverage.profdata \
                    -ignore-filename-regex='.*(cargo/registry|xxh3/fuzz|rustc).*' \
                    > fuzz/coverage/coverage_report.html
	$(LLVM_COV) report fuzz/target/*/release/fuzz_xxh3 \
                    -instr-profile=fuzz/coverage/fuzz_xxh3/coverage.profdata \
                    -ignore-filename-regex='.*(cargo/registry|xxh3/fuzz|rustc).*'
	firefox ./fuzz/coverage/coverage_report.html
