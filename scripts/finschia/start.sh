#!/bin/bash
set -o errexit -o nounset -o pipefail

# Please keep this in sync with the Ports overview in HACKING.md
TENDERMINT_PORT_GUEST="26657"
TENDERMINT_PORT_HOST="26657"
API_PORT_GUEST="1317"
API_PORT_HOST="1317"
GRPC_PORT_GUEST="9090"
GRPC_PORT_HOST="9090"

# If this error `realpath: command not found` occurs, run the following command.
# brew install coreutils
SCRIPT_DIR="$(realpath "$(dirname "$0")")"
source "$SCRIPT_DIR"/env

if [[ -z $1 ]]; then
  echo "CONFIG_DIR does not exists"
  exit 1
else
  CONFIG_DIR=$1
fi

echo "Using temporary dir $CONFIG_DIR" >&2
FINSCHIA_LOGFILE="$CONFIG_DIR/finschia.log"

# Use a fresh volume for every start
docker volume rm -f fnsad_data
CUR_PATH="$(realpath "$(dirname "$0")")"
# test contract
INTEG_TEST_DIR=${CUR_PATH}"/../"
# wasm
ARTIFACTS=${CUR_PATH}"/../../artifacts"

cp "$SCRIPT_DIR/run_finschia.sh" "/$CONFIG_DIR/run_finschia.sh"
cp "/$CONFIG_DIR/run_finschia.sh" "/tmp/run_finschia.sh"

docker run --rm \
  --name "$CONTAINER_NAME" \
  -p "$TENDERMINT_PORT_HOST":"$TENDERMINT_PORT_GUEST" \
  -p "$API_PORT_HOST":"$API_PORT_GUEST" \
  -p "$GRPC_PORT_HOST":"$GRPC_PORT_GUEST" \
  -v "$INTEG_TEST_DIR":"/root/scripts" \
  -v "$ARTIFACTS":"/root/artifacts" \
  --mount type=bind,source="$CONFIG_DIR",target="/tmp" \
  --mount type=volume,source=fnsad_data,target=/root \
  "$REPOSITORY:$VERSION" \
  "/tmp/run_finschia.sh" \
  >"$FINSCHIA_LOGFILE" 2>&1 &

echo "fnsad running on http://localhost:$TENDERMINT_PORT_HOST and logging into $FINSCHIA_LOGFILE" >&2
if [ -n "${CI:-}" ]; then
  # Give process some time to come alive. No idea why this helps. Needed for CI.
  sleep 0.5

  # Follow the logs in CI's background job
  tail -f "$FINSCHIA_LOGFILE"
fi
