#!/usr/bin/env bash
set -ex

mode="mainnet"

SCRIPT_DIR="$(realpath "$(dirname "$0")")"
source "$SCRIPT_DIR"/env

CHAIN_ID="simd-testing"
MONIKER="simd-testing"
if [[ -z $1 ]]; then
  echo "CONFIG_DIR does not exists"
  exit 1
else
  CONFIG_DIR=$1"/.finschia"
fi

CHAIN_DIR=${CONFIG_DIR}

FNSAD=${FNSAD:-fnsad}
# initialize
rm -rf $CONFIG_DIR

# Initialize configuration files and genesis file
# moniker is the name of your node
${FNSAD} init simd-testing --chain-id=$CHAIN_ID --home=${CHAIN_DIR}

# Please do not use the TEST_MNEMONIC for production purpose
TEST_MNEMONIC="mind flame tobacco sense move hammer drift crime ring globe art gaze cinnamon helmet cruise special produce notable negative wait path scrap recall have"
N=9

# generate normal account keys
for ((i = 0; i < N; i++))
do
  ${FNSAD} keys add account${i} --home=${CHAIN_DIR} --keyring-backend=test --recover --index=${i} <<< ${TEST_MNEMONIC}
done
# generate multisig key
${FNSAD} keys add multisig0 --home=${CHAIN_DIR} --keyring-backend=test --multisig account0,account1,account2,account3,account4 --multisig-threshold 2
# generate validator key
${FNSAD} keys add validator0 --home=${CHAIN_DIR} --keyring-backend=test --recover --account=1 <<< ${TEST_MNEMONIC}


# Add both accounts, with coins to the genesis file
for ((i = 0; i < N; i++))
do
  ${FNSAD} add-genesis-account $(${FNSAD} keys show account${i} -a --home=${CHAIN_DIR} --keyring-backend=test) 100000000000cony,20000000000stake --home=${CHAIN_DIR}
done
${FNSAD} add-genesis-account $(${FNSAD} keys show multisig0 -a --home=${CHAIN_DIR} --keyring-backend=test) 100000000000cony,20000000000stake --home=${CHAIN_DIR}
${FNSAD} add-genesis-account $(${FNSAD} keys show validator0 -a --home=${CHAIN_DIR} --keyring-backend=test) 100000000000cony,20000000000stake --home=${CHAIN_DIR}

${FNSAD} gentx validator0 10000000000stake --home=${CHAIN_DIR} --keyring-backend=test --chain-id=$CHAIN_ID --moniker=$MONIKER

${FNSAD} collect-gentxs --home=${CHAIN_DIR}

${FNSAD} validate-genesis --home=${CHAIN_DIR}
