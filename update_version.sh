#!/usr/bin/env bash

NEW_VERSION=$1

perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" */**/Cargo.toml
echo "$NEW_VERSION" > python/snips_nlu_utils/__version__
