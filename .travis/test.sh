#!/usr/bin/env bash
source .travis/common.sh

export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"
perl -p -i -e "s/^snips-nlu-utils = .*\$/snips-nlu-utils = { path = \"..\/..\" \}/g" */**/Cargo.toml

if [[ "${RUST_TESTS}" == "true" ]]; then
  echo "Rust tests..."
  cargo test --all || die "Rust tests failed"
fi

if [[ "${PYTHON_TESTS}" == "true" ]]; then
  echo "Python tests..."
  cd python
  python -m pip install tox
  tox || die "Python tests failed"
  cd ..
fi
