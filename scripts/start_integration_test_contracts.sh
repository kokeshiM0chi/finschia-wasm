#!/bin/sh

# Finschia app start
CONFIG_DIR=$(mktemp -d)
./finschia/setup.sh ${CONFIG_DIR}
./finschia/start.sh ${CONFIG_DIR}

# wait docker start
timeout 60 bash -c "until docker ps | awk '{print $NF}' | grep finschia-wasm-testing-app > /dev/null; do sleep 0.5; done"

# install for testing tool
docker exec finschia-wasm-testing-app apk add --no-cache jq bash curl

# execute test
docker exec -it finschia-wasm-testing-app /root/scripts/integration_test_contracts.sh /bin/sh

# stop docker container
docker container stop finschia-wasm-testing-app
