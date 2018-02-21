#!/usr/bin/env bash
source ./.travis/common.sh

NEW_VERSION=$(parseRustVersion)
echo "Updating versions to version ${NEW_VERSION}"
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" */**/Cargo.toml
perl -p -i -e "s/https:\/\/github\.com\/snipsco\/snips-nlu-utils\", tag = \".*\"/https:\/\/github\.com\/snipsco\/snips-nlu-utils\", tag = \"$NEW_VERSION\"/g" */**/Cargo.toml
echo "$NEW_VERSION" > python/snips_nlu_utils/__version__
