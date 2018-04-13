#!/usr/bin/env bash

NEW_VERSION=${1?"usage $0 <new version>"}

echo "Updating versions to version ${NEW_VERSION}"
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" Cargo.toml
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" */**/Cargo.toml
echo "$NEW_VERSION" > python/snips_nlu_utils/__version__

if [[ "${NEW_VERSION}" == "${NEW_VERSION/-SNAPSHOT/}" ]]
then
    perl -p -i -e "s/^snips-nlu-utils = .*\$/snips-nlu-utils = { git = \"https:\/\/github.com\/snipsco\/snips-nlu-utils\", tag = \"${NEW_VERSION}\" }/g" */**/Cargo.toml
else
    perl -p -i -e "s/^snips-nlu-utils = .*\$/snips-nlu-utils = { path = \"..\/..\" \}/g" */**/Cargo.toml
fi
