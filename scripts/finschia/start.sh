#!/bin/bash
set -o errexit -o nounset -o pipefail

# Please keep this in sync with the Ports overview in HACKING.md
TENDERMINT_PORT_GUEST="26658"
TENDERMINT_PORT_HOST="26658"
API_PORT_GUEST="1317"
API_PORT_HOST="1317"
GRPC_PORT_GUEST="9090"
GRPC_PORT_HOST="9090"

# If this error `realpath: command not found` occurs, run the following command.
# brew install coreutils
SCRIPT_DIR="$(realpath "$(dirname "$0")")"
source "$SCRIPT_DIR"/env
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/fnsa.XXXXXXXXX")
chmod 755 "$TMP_DIR"
echo "Using temporary dir $TMP_DIR"
FINSCHIA_LOGFILE="$TMP_DIR/finschia.log"

# Use a fresh volume for every start
docker volume rm -f fnsad_data
CUR_PATH="$(realpath "$(dirname "$0")")"
# test contract
INTEG_TEST_DIR=${CUR_PATH}"/../"
# wasm
ARTIFACTS=${CUR_PATH}"/../../artifacts"

docker run --rm \
  --name "$CONTAINER_NAME" \
  -p "$TENDERMINT_PORT_HOST":"$TENDERMINT_PORT_GUEST" \
  -p "$API_PORT_HOST":"$API_PORT_GUEST" \
  -p "$GRPC_PORT_HOST":"$GRPC_PORT_GUEST" \
  -v "$INTEG_TEST_DIR":"/root/scripts" \
  -v "$ARTIFACTS":"/root/artifacts" \
  --mount type=bind,source="$SCRIPT_DIR/template",target=/template \
  --mount type=volume,source=fnsad_data,target=/root \
  "$REPOSITORY:$VERSION" \
  /template/run_finschia.sh \
  >"$FINSCHIA_LOGFILE" >&2 &

echo "fnsad running on http://localhost:$TENDERMINT_PORT_HOST and logging into $FINSCHIA_LOGFILE"
if [ -n "${CI:-}" ]; then
  # Give process some time to come alive. No idea why this helps. Needed for CI.
  sleep 0.5

  # Follow the logs in CI's background job
  tail -f "$FINSCHIA_LOGFILE"
fi
