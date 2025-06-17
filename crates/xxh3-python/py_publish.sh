#!/bin/bash

PYTHON3=/opt/python/cp311-cp311/bin/python3

cp -r /xxh3-ro /xxh3
cd /xxh3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=1.85.0
source $HOME/.cargo/env

cd /tmp
$PYTHON3 -m venv venv
cd /xxh3/crates/xxh3-python
source /tmp/venv/bin/activate
python3 -m pip install --upgrade pip
python3 -m pip install maturin toml

# xargs is just to merge the lines together into a single line
python3 -m maturin publish -i $(ls -1 /opt/python/*/bin/python3 | xargs | sed 's/ / -i /g')
