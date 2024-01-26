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

# This is a function that checks if executeMsg is successful.
check_run_info() {
	if [[ "$raw_log" == *"failed"* ]]; then
		echo -e "$1$raw_log"
		exit 1
	fi
}

#*** store collection contract ***
result=$(fnsad tx wasm store ${artifacts_path}/collection.wasm --node ${URL} --from ${FROM_ACCOUNT} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `store`
raw_log=$(echo ${result} | jq .raw_log)
check_run_info "store: "

#*** instantiate collection contract ***
result=$(fnsad tx wasm instantiate 1 '{"name":"collection_name","uri":"collection_uri","meta":"collection_meta", "owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}' --label collection1 --admin link146asaycmtydq45kxc8evntqfgepagygelel00h  --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node ${URL} ${CHAIN_OPTION})

## confirm a result of `instantiate`
raw_log=$(echo ${result} | jq .raw_log)
check_run_info "instantiate: "

#*** issue_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"issue_nft":{"name":"nft1_name","meta":"nft1_meta","owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `issue_nft`
raw_log=$(echo ${result} | jq .raw_log)
check_run_info "execute; issue_nft: "

#*** mint_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"mint_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","params":[{"token_type":"10000001","name":"nft1_name1","meta":"nft1_meta1"},{"token_type":"10000001","name":"nft1_name2","meta":"nft1_meta2"}]}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `mint_nft`
raw_log=$(echo ${result} | jq .raw_log)
check_run_info "execute; mint_nft: "

#*** send_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"send_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `send_nft`
raw_log=$(echo ${result} | jq .raw_log)
check_run_info "execute; send_nft"

#*** burn_nft collection contract ***
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"burn_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --node ${URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `burn_nft`
raw_log=$(echo ${result} | jq .raw_log)
check_run_info "execute; burn_nft"
