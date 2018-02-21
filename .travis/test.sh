#!/usr/bin/env bash
source .travis/common.sh

echo "Rust tests..."
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"
cargo test --all || die "Rust tests failed"

echo "Python tests..."
perl -p -i -e "s/^snips-nlu-utils = .*\$/snips-nlu-utils = { path = \"..\/..\" \}/g" */**/Cargo.toml
cd python
python -m pip install tox
tox || die "Python tests failed"