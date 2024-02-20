#!/bin/bash

# Get the latest Finschia tag
TAG=$(curl "https://api.github.com/repos/Finschia/finschia/tags" | jq -r '.[0].name')

# Copy init_single.sh from Finschia/finschia/init_single.sh
init_single=$(mktemp)
curl "https://raw.githubusercontent.com/Finschia/finschia/${TAG}/init_single.sh" -o $init_single

# Remove v from tag prefix
# ex. v1.0.0 -> 1.0.0
TAG=$(echo "$TAG" | cut -c 2-)
TEST_DOCKER_IMAGE=finschia/finschianode:${TAG}
docker pull ${TEST_DOCKER_IMAGE}

# run Finschia/finschia/init_single.sh
env FNSAD="docker run -i --rm -p 26656:26656 -p 26657:26657 -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad" bash $init_single

container_id=$(docker run -d -p 26656:26656 -p 26657:26657 -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad start)

script_dir=$(realpath $(dirname $0))
docker exec ${container_id} apk add --no-cache jq bash curl &&
docker cp ${script_dir}/integration_test_contracts.sh ${container_id}:/root/integration_test_contracts.sh &&
docker cp ${script_dir}/../artifacts ${container_id}:/root/artifacts &&
docker exec ${container_id} /root/integration_test_contracts.sh

docker stop ${container_id}

# Remove the temporary file `Finschia/init_single.sh`
rm $init_single

