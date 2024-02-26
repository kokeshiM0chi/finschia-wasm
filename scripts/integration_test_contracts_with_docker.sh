#!/bin/bash

# Commands
DOCKER=${DOCKER:-docker}
JQ=${JQ:-jq}
VERBOSE=${VERBOSE:-false}

script_dir=$(realpath $(dirname $0))

# Build collection.wasm if not exitst
echo "##### Build collection.wasm if there is not #####" >&2
env DOCKER=${DOCKER} ${script_dir}/build_collection.sh >&2

# Get the latest Finschia tag
tag=$(curl -s "https://api.github.com/repos/Finschia/finschia/tags" | ${JQ} -r '.[0].name')

# Remove v from tag prefix
# ex. v1.0.0 -> 1.0.0
tag_num=$(echo "${tag}" | cut -c 2-)

echo "##### Pull finschianode docker image #####" >&2

image=finschia/finschianode:${tag_num}
${DOCKER} pull ${image} >&2

echo "##### Create and start finschianode docker container #####" >&2
container=$(${DOCKER} run -id ${image})

echo "##### Install tools to container #####" >&2
${DOCKER} exec ${container} apk add --no-cache jq bash curl >&2 &&

echo "##### Init node for tests #####" >&2
${DOCKER} exec ${container} curl "https://raw.githubusercontent.com/Finschia/finschia/${tag}/init_single.sh" -o init_single.sh
${DOCKER} exec ${container} bash init_single.sh >&2

echo "##### Start node #####" >&2

${DOCKER} exec -d ${container} fnsad start >&2

echo "##### Install scripts and contracts to container #####" >&2 &&
${DOCKER} cp ${script_dir}/integration_test_contracts.sh ${container}:/root/integration_test_contracts.sh >&2 &&
${DOCKER} cp -L ${script_dir}/../artifacts/collection.wasm ${container}:/root/ >&2 &&
echo "##### Start tests #####" >&2 &&
${DOCKER} exec ${container} env VERBOSE=${VERBOSE} /root/integration_test_contracts.sh

echo "##### Remove docker container #####" >&2
${DOCKER} rm -f ${container} >&2
