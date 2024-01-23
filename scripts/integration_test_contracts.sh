#!/bin/bash -e

# This sleep is the sum of the time when docker is pulled, started,
# and the height of the node increases to 1 or more.
sleep 30

# send transaction to node
cur_path=`pwd`
chain_dir="${cur_path}/finschia/template/.finschia"
artifacts_path="${cur_path}/../artifacts/"

# This is a function that checks if the result is as expected.
check_result() {
    if [[ "${expected_key}" = "${key}" ]]; then echo "ok"; else echo "ng"; fi
    if [[ "${expected_val}" = "${value}" ]]; then echo "ok"; else echo "ng"; fi
}

#*** store collection contract ***
# store contract
result=$(fnsad tx wasm store ${artifacts_path}/collection.wasm --node http://localhost:26658 --home=${chain_dir} --from link146asaycmtydq45kxc8evntqfgepagygelel00h --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

# confirm a result of `store`
key=$(echo "${result}" | jq '.logs[] | .events' | jq '.[0]' | jq '.attributes[2]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events' | jq '.[0]' | jq '.attributes[2]' | jq '.value')
expected_key='"module"'
expected_val='"wasm"'

check_result

#*** instantiate collection contract ***
# instantiate contract
result=$(fnsad tx wasm instantiate 1 '{"name":"collection_name","uri":"collection_uri","meta":"collection_meta", "owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}' --label collection1 --admin link146asaycmtydq45kxc8evntqfgepagygelel00h  --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node http://localhost:26658 --home=${chain_dir} --chain-id simd-testing --keyring-backend test -b block -o json -y)

# confirm a result of `instantiate`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[1]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[1]' | jq '.value')
expected_key='"code_id"'
expected_val='"1"'

check_result

#*** issue_nft collection contract ***
# issue a nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"issue_nft":{"name":"nft1_name","meta":"nft1_meta","owner":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node http://localhost:26658 --home=${chain_dir} --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

# confirm a result of `issue_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'

check_result

#*** mint_nft collection contract ***
# mint a nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"mint_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","params":[{"token_type":"10000001","name":"nft1_name1","meta":"nft1_meta1"},{"token_type":"10000001","name":"nft1_name2","meta":"nft1_meta2"}]}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --node http://localhost:26658 --home=${chain_dir} --chain-id simd-testing --keyring-backend test --gas 10000000 -b block -o json -y)

# confirm a result of `mint_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'

check_result

#*** send_nft collection contract ***
# send_nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"send_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","to":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --keyring-backend test --node http://localhost:26658 --home=${chain_dir} --chain-id simd-testing --gas 10000000 -b block -o json -y)

# confirm a result of `send_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'

check_result

#*** burn_nft collection contract ***
# burn_nft
result=$(fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"burn_nft":{"from":"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8","token_ids":["1000000100000001"]}}' --from link146asaycmtydq45kxc8evntqfgepagygelel00h --keyring-backend test --node http://localhost:26658 --home=${chain_dir} --chain-id simd-testing --gas 10000000 -b block -o json -y)

# confirm a result of `burn_nft`
key=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.key')
value=$(echo "${result}" | jq '.logs[] | .events[0]' | jq '.attributes[0]' | jq '.value')
expected_key='"_contract_address"'
expected_val='"link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8"'

check_result
