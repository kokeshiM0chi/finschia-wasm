#!/bin/bash

set -e

# commands
DOCKER=${DOCKER:-docker}

# Build collection contract
WORKSPACE_DIR=$(realpath $(dirname $0)/../)
TARGET="${WORKSPACE_DIR}/artifacts/collection.wasm"

# if arm64, add it to container name's suffix
SUFFIX=""
if [ "$(uname -m)" = "arm64" ]; then
  SUFFIX="-arm64"
  ln -sf "${WORKSPACE_DIR}/artifacts/collection-aarch64.wasm" ${TARGET}
fi

if [ ! -f "${TARGET}" ]; then
  echo "##### Build collection.wasm #####" >&2
  ${DOCKER} run --rm -v "${WORKSPACE_DIR}":/code \
    --mount type=volume,source="devcontract_cache_collection",target=/code/examples/contracts/collection/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer${SUFFIX}:0.15.0 ./examples/contracts/collection
fi
