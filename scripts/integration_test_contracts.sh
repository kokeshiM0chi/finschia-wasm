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
# This is a function that checks if the result is as expected.
check_result() {
    if [[ "$result" == *"failed"* ]]; then
      exit 1
    fi
}

#*** store collection contract ***
## store contract
result=$(fnsad tx wasm store ${artifacts_path}/collection.wasm --node http://localhost:26658 --from link146asaycmtydq45kxc8evntqfgepagygelel00h --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

## confirm a result of `store`
check_result

#*** instantiate collection contract ***
## instantiate contract
result=$(fnsad tx wasm instantiate 1 '{"name":"collection_name","uri":"collection_uri","meta":"collection_meta", "owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}' --label collection1 --admin link146asaycmtydq45kxc8evntqfgepagygelel00h  --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node http://localhost:26658 --chain-id simd-testing --keyring-backend test -b block -o json -y)

## confirm a result of `instantiate`
check_result

#*** issue_nft collection contract ***
## issue a nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"issue_nft":{"name":"nft1_name","meta":"nft1_meta","owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node http://localhost:26658 --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

## confirm a result of `issue_nft`
check_result

#*** mint_nft collection contract ***
## mint a nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"mint_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","params":[{"token_type":"10000001","name":"nft1_name1","meta":"nft1_meta1"},{"token_type":"10000001","name":"nft1_name2","meta":"nft1_meta2"}]}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node http://localhost:26658 --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

## confirm a result of `mint_nft`
check_result

#*** send_nft collection contract ***
## send_nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"send_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --keyring-backend test --node http://localhost:26658 --chain-id simd-testing --gas 10000000 -b block -o json -y)

## confirm a result of `send_nft`
check_result

#*** burn_nft collection contract ***
## burn_nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"burn_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --keyring-backend test --node http://localhost:26658 --chain-id simd-testing --gas 10000000 -b block -o json -y)

## confirm a result of `burn_nft`
check_result
