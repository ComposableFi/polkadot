#! /bin/bash

REPO=composablefi
NAME=mmr-polkadot
# BRANCH=$1
# TAG=$(echo $BRANCH | sed -E "s/^(.+)-(v.+)$/\2/") # "mmr-polkadot-v0.9.24" -> "v0.9.24"

BRANCH="mmr-polkadot-v0.9.24"
TAG="test" # tmp

if [ -z "$BRANCH" ]; then
    echo -e "usage:\n$0 <branch name>"
    exit 1
fi

echo "branch name: $BRANCH, tag: $TAG"

docker build -f docker/mmr-polkadot.dockerfile \
    --build-arg BRANCH=$BRANCH \
    -t "${REPO}/${NAME}:${TAG}" \
    .

docker push "${REPO}/${NAME}:${TAG}"
