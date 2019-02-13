#!/bin/sh

set -ex

VERSION=$1

if [[ -z "$VERSION" ]]
then
    echo "Usage: $0 <version>"
    exit 1
fi


./update_version.sh ${VERSION}

git commit . -m "Set post-release version to $VERSION"
