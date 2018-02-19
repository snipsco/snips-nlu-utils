#!/usr/bin/env bash
source .travis/common.sh

rustBuildAndTest () {
    echo "Rust build and test"

    export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"

    cargo build --all  || \
        die "Rust build failed"

    cargo test --all || \
        die "Rust test failed"
}

pythonBuild () {
    local pythonPath=$1
    local venvPath=$2

    echo "Python installation..."
    cd python
    rm -rf ${venvPath}
    virtualenv -p ${pythonPath} ${venvPath} || \
        die "Failed to create virtualenv"

    . ${venvPath}/bin/activate
    pip install -r requirements.txt  || \
        die "Failed to install requirements"

    echo "Python build..."
    ssh-agent sh -c "ssh-add; pip install -e . --verbose" || \
        die "Failed to install Python"
}

updateVersions ${TAG_VERSION}

rustBuildAndTest

pythonBuild ${PYTHON_PATH} ${VENV_PATH}