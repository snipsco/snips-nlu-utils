#!/usr/bin/env bash
source .travis/common.sh

MASTER_BRANCH="master"
DEVELOP_BRANCH="develop"
BRANCH=${TRAVIS_BRANCH_NAME}

branchIsMergedInto() {
    local subject=$1
    local base=$2
    local allMerges="$(git branch --no-color --contains ${subject} | sed 's/^[* ] //')"
    has ${base} ${allMerges}
}

gitAllTags() { git tag; }

gitCurrentBranch() {
    git branch --no-color | grep '^\* ' | grep -v 'no branch' | sed 's/^* //g'
}

gitTagExists() {
    has $1 $(gitAllTags)
}

updateAndCommitVersions() {
    local pythonVersionPath=$1
    local pythonVersion="$(head -n 1 ${pythonVersionPath})"

    if [[ ${pythonVersion} != ${TAG_VERSION} ]]; then
        updateVersions ${TAG_VERSION}
        echo "Commit changes..."
        # Commit the version update
        git add python  || \
            die "Failed to add changes"
        git commit -m "Update Python tag version to $TAG_VERSION" || \
            die "Failed to commit version update"
    fi
}

mergeIntoMasterBranch() {
    # try to merge into master
    # in case a previous attempt to finish this release branch has failed,
    # but the merge into master was successful, we skip it now
    if ! branchIsMergedInto "$BRANCH" "$MASTER_BRANCH"; then
        git checkout "$MASTER_BRANCH" || \
            die "Could not check out $MASTER_BRANCH."
        git merge --no-ff ${BRANCH} || \
            die "There were merge conflicts."
    fi
}

performTag() {
    # try to tag the release
    # in case a previous attempt to finish this release branch has failed,
    # but the tag was set successful, we skip it now
    if ! gitTagExists "$TAG_VERSION"; then
        git tag -a "$TAG_VERSION" -m "Release ${BRANCH}" || \
            die "Tagging failed. Please run finish again to retry."
    fi
}

mergeIntoDevelopBranch() {
    if ! branchIsMergedInto "$BRANCH" "$DEVELOP_BRANCH"; then
        git checkout "$DEVELOP_BRANCH" || \
          die "Could not check out $DEVELOP_BRANCH."

        git merge --no-ff "$BRANCH" || \
            die "There were merge conflicts."
    fi

}

deleteBranch() {
    if [ "$BRANCH" == "$(gitCurrentBranch)" ]; then
        git checkout "$MASTER_BRANCH"
    fi
    git branch -D "$BRANCH" || \
        die "Failed to delete branch"
}

publish() {
    git push origin "$DEVELOP_BRANCH" || \
        die "Could not push to $DEVELOP_BRANCH from origin."
    git push origin "$MASTER_BRANCH" || \
        die "Could not push to $MASTER_BRANCH from origin."
    git push --tags origin || \
      die "Could not push tags to origin."
    git push origin :"$BRANCH" || \
        die "Could not delete the remote $BRANCH in origin."
}

uploadAsset() {
    local venvPath=$1
    local asset=$2

    cd python
    . ${venvPath}/bin/activate
    pip install twine
    ssh-agent sh -c "ssh-add; python setup.py ${asset}; twine upload -u ${PYPI_USER} -p ${PYPI_ENCRYPTED_PWD}" || \
      die "Failed to build and and upload asset"
}

echo ${BRANCH}
if  [[ ${BRANCH} == release/* ]] || [[ ${BRANCH} == hotfix* ]];then
    echo "Performing release..."
    git stash
    git config --global user.email 'tobor.spins@snips.net'
    git config --global user.name 'Tobor'

    # Align versions
    updateAndCommitVersions "python/snips_nlu_utils/__version__"

    # Merge into master
    echo "Merging build branch into master..."
    mergeIntoMasterBranch

    # Perform tag
    echo "Tagging..."
    performTag

    # Merge into develop
    echo "Merging master into development branch..."
    mergeIntoDevelopBranch

    # Delete branch
    echo "Deleting current branch..."
    deleteBranch
    
    # Publish code
    echo "Publishing code on Github..."
    publish

    # Build and publish Python wheel
    echo "Uploading python wheel..."
    uploadAsset ${VENV_PATH} bdist_wheel

    # Publish source distribution only once
    if [ ${TRAVIS_OS_NAME} == "*osx" ] && [${PYTHON_VERSION} == "2.7"]; then
        echo "Uploading source distribution..."
        uploadAsset ${VENV_PATH} sdist
    fi
fi