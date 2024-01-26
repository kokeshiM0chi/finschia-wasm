#!/bin/sh

# Wait until the height of the block is greater than or equal to 1.
h="0"
until [[ ${h} == '"1"' ]]
do
  h=$(fnsad query block 1 --node http://localhost:26658 --chain-id simd-testing --keyring-backend test | jq .block.header.height)
done

# send transaction to node
cur_path=`pwd`

artifacts_path="artifacts"

## parameters
CONTRACT_ADDRESS="link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"
FROM_ACCOUNT="link146asaycmtydq45kxc8evntqfgepagygelel00h"
URL="http://localhost:26658"
CODE_ID=1

# This is a function that checks if the result is as expected.
check_result() {
    if [[ "$result" == *"failed"* ]]; then
      exit 1
    fi
}

#*** store collection contract ***
result=$(fnsad tx wasm store ${artifacts_path}/collection.wasm --node ${URL} --from ${FROM_ACCOUNT} --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

## confirm a result of `store`
check_result

#*** instantiate collection contract ***
result=$(fnsad tx wasm instantiate ${CODE_ID} '{"name":"collection_name","uri":"collection_uri","meta":"collection_meta", "owner":${CONTRACT_ADDRESS}}' --label collection1 --admin ${FROM_ACCOUNT}  --from ${FROM_ACCOUNT} --node ${URL} --chain-id simd-testing --keyring-backend test -b block -o json -y)

## confirm a result of `instantiate`
check_result

#*** issue_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"issue_nft":{"name":"nft1_name","meta":"nft1_meta","owner":${CONTRACT_ADDRESS}}}' --from ${FROM_ACCOUNT} --node ${URL} --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

## confirm a result of `issue_nft`
check_result

#*** mint_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"mint_nft":{"from":${CONTRACT_ADDRESS},"to":${CONTRACT_ADDRESS},"params":[{"token_type":"10000001","name":"nft1_name1","meta":"nft1_meta1"},{"token_type":"10000001","name":"nft1_name2","meta":"nft1_meta2"}]}}' --from ${FROM_ACCOUNT} --node ${URL} --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

## confirm a result of `mint_nft`
check_result

#*** send_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"send_nft":{"from":${CONTRACT_ADDRESS},"to":${CONTRACT_ADDRESS},"token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --keyring-backend test --node ${URL} --chain-id simd-testing --gas 10000000 -b block -o json -y)

## confirm a result of `send_nft`
check_result

#*** burn_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"burn_nft":{"from":${CONTRACT_ADDRESS},"token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --keyring-backend test --node ${URL} --chain-id simd-testing --gas 10000000 -b block -o json -y)

## confirm a result of `burn_nft`
check_result
