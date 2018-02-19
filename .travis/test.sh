#!/usr/bin/env bash
source .travis/common.sh

echo "Rust tests..."
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"
cargo test --all || \
        die "Rust test failed"

echo "Python tests..."
cd python
. ${VENV_PATH}/bin/activate
ssh-agent sh -c "ssh-add; pip install -e '.[test]' --verbose" || \
        die "Failed to install test dependencies"
tox