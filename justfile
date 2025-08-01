build: pre
    cargo build --all-targets
    cargo doc

pre:
    cargo deny check licenses
    cargo fmt --all -- --check
    cargo clippy --all --all-targets

bench: pre
    cargo bench --bench benchmark_128
    firefox ./target/criterion/report/index.html

flamegraph:
    cargo flamegraph --bench benchmark_128 -- --bench
    firefox ./flamegraph.svg

publish_py: test_py
    docker pull quay.io/pypa/manylinux2014_x86_64
    docker run -it --rm -v `pwd`:/xxh3-ro:ro quay.io/pypa/manylinux2014_x86_64 /xxh3-ro/crates/xxh3-python/py_publish.sh

py_venv:
    [ -d venv ] || python3 -m venv venv
    . ./venv/bin/activate && pip install -U maturin

test_py: install_py
    . ./venv/bin/activate && python3 -m unittest discover --start-directory=./crates/xxh3-python

install_py: pre py_venv
    . ./venv/bin/activate && maturin develop --manifest-path=./crates/xxh3-python/Cargo.toml

test: pre
    RUST_BACKTRACE=1 cargo test

fuzz: pre
    cargo fuzz run --sanitizer=none fuzz_xxh3 -- -max_len=1000000

fuzz_ci: pre
    cargo fuzz run --sanitizer=none fuzz_xxh3 -- -max_len=1000000 -max_total_time=60

fuzz_coverage: pre
    #!/usr/bin/env bash
    set -euxo pipefail
    RUST_SYSROOT=`cargo rustc -- --print sysroot 2>/dev/null`
    LLVM_COV=`find $RUST_SYSROOT -name llvm-cov`
    echo $(LLVM_COV)
    rustup component add llvm-tools-preview
    cargo fuzz coverage --sanitzer=none fuzz_xxh3
    $(LLVM_COV) show fuzz/target/*/release/fuzz_xxh3 \
                    --format html \
                    -instr-profile=fuzz/coverage/fuzz_xxh3/coverage.profdata \
                    -ignore-filename-regex='.*(cargo/registry|xxh3/fuzz|rustc).*' \
                    > fuzz/coverage/coverage_report.html
    $(LLVM_COV) report fuzz/target/*/release/fuzz_xxh3 \
                    -instr-profile=fuzz/coverage/fuzz_xxh3/coverage.profdata \
                    -ignore-filename-regex='.*(cargo/registry|xxh3/fuzz|rustc).*'
    firefox ./fuzz/coverage/coverage_report.html
