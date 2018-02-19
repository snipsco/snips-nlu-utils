#!/usr/bin/env bash

PYTHON_PATH=$(which python"$PYTHON_VERSION")
COMMIT_ID=$(git rev-parse --short HEAD)
VENV_PATH="/tmp/venv27-$COMMIT_ID"

warn() { echo "$@" >&2; }

die() { warn "$@"; exit 1; }

escape() {
	echo "$1" | sed 's/\([\.\$\*]\)/\\\1/g'
}

has() {
	local item=$1; shift
	echo " $@ " | grep -q " $(escape $item) "
}

parseRustVersion() {
    grep -w Cargo.toml -e '^version = ".*' | sed -- 's/version = "//g' | sed -- 's/"//g'
}

updateVersions() {
    local tagVersion=$1
    echo "Updating version..."
    ssh-agent sh -c "ssh-add; ./update_version.sh $tagVersion" || \
        die "Could not upload version"
}

TAG_VERSION=$(parseRustVersion)