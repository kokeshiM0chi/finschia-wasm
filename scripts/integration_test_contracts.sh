#!/bin/bash

## parameters
CONTRACT_ADDRESS='link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8'
FROM_ACCOUNT='link146asaycmtydq45kxc8evntqfgepagygelel00h'
URL='http://localhost:26658'
CHAIN_OPTION='--chain-id simd-testing --keyring-backend test -b block -o json -y'

## Wait until the height of the block is greater than or equal to 1.
timeout 60 bash -c "until fnsad query block 1 --node ${URL} --chain-id simd-testing > /dev/null; do sleep 0.5; done"
# If the timeout fails, the process will be terminated abnormally.
exitstatus=$?
if [[ $exitstatus -ne 0 ]]; then
    echo "timeout failed"
    exit 1
fi
# send transaction to node
cur_path=`pwd`

artifacts_path="artifacts"
# This is a function that checks if the result is as expected.
check_result() {
    if [[ "$1" != "${expected_key}" || "$2" != "${expected_val}" ]]; then echo "Error: $3"; exit 1; fi
}

#*** store collection contract ***
result=$(fnsad tx wasm store ${artifacts_path}/collection.wasm --node ${URL} --from ${FROM_ACCOUNT} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `store`
key=$(echo "${result}" | jq '.logs[] | .events' | jq '.[0]' | jq '.attributes[2]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events' | jq '.[0]' | jq '.attributes[2]' | jq '.value')
expected_key='"module"'
expected_val='"wasm"'
check_result "${key}" "${value}" "${result}"

#*** instantiate collection contract ***
result=$(fnsad tx wasm instantiate 1 '{"name":"collection_name","uri":"collection_uri","meta":"collection_meta", "owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}' --label collection1 --admin link146asaycmtydq45kxc8evntqfgepagygelel00h  --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node ${URL} ${CHAIN_OPTION})

## confirm a result of `instantiate`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[1]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[1]' | jq '.value')
expected_key='"code_id"'
expected_val='"1"'
check_result "${key}" "${value}" "${result}"

#*** issue_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"issue_nft":{"name":"nft1_name","meta":"nft1_meta","owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `issue_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'
check_result "${key}" "${value}" "${result}"

#*** mint_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"mint_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","params":[{"token_type":"10000001","name":"nft1_name1","meta":"nft1_meta1"},{"token_type":"10000001","name":"nft1_name2","meta":"nft1_meta2"}]}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `mint_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'
check_result "${key}" "${value}" "${result}"

#*** send_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"send_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `send_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'
check_result "${key}" "${value}" "${result}"

#*** burn_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"burn_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `burn_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'
check_result "${key}" "${value}" "${result}"
