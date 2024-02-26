#!/bin/bash

## parameters
COLLECTION_CONTRACT=${COLLECTION_CONTRACT:-"collection.wasm"}
VERBOSE=${VERBOSE:-"false"}
FROM_ACCOUNT=${FROM_ACCOUNT:-"link146asaycmtydq45kxc8evntqfgepagygelel00h"}
NODE_URL=${NODE_URL:-"http://localhost:26657"}
CHAIN_OPTION=${CHAIN_OPTION:-"--chain-id finschia --keyring-backend test -b block -o json -y"}

## Wait until the height of the block is greater than or equal to 1.
echo "##### Waiting finschia node starts #####" >&2
timeout 60 bash -c 'until [[ $(curl -s "'${NODE_URL}'/status?" | jq -r ".result.sync_info.latest_block_height // 0") -ge 1 ]]; do sleep 0.5; done'

# If the timeout fails, the process will be terminated abnormally.
exitstatus=$?
if [[ $exitstatus -ne 0 ]]; then
  echo "waiting finschia node is timeout" >&2
  exit 1
fi

set -e

echo "Confirmed a node is running" >&2

# This checks the result is succeed/failed and output it.
# If it is failed, exit.
judge_result_and_output() {
  local test_name="$1"
  local res="$2"
  if [[ "${res}" == *"failed"* ]]; then
    echo -e "${test_name}\tFAIL"
    echo "========" >&2
    echo "FAIL WITH" >&2
    echo "${res}" >&2
    echo "========" >&2
    exit 1
  else
    echo -e "${test_name}\tPASS"
    if [ "${VERBOSE}" != "false" ]; then
      echo "========" >&2
      echo "PASS WITH" >&2
      echo "${res}" >&2
      echo "========" >&2
    fi
  fi
}

# send transaction to node

echo "##### TEST: store collection contract #####" >&2
result=$(fnsad tx wasm store ${COLLECTION_CONTRACT} --node ${NODE_URL} --from ${FROM_ACCOUNT} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `store`
raw_log=$(echo ${result} | jq .raw_log)
judge_result_and_output "store" "${raw_log}"

echo "##### TEST: instantiate collection contract #####" >&2
CODE_ID=$(echo "${result}" | jq '.logs[] | select(.msg_index == 0) | .events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value | tonumber')
result=$(fnsad tx wasm instantiate ${CODE_ID} '{"name":"collection_name","uri":"collection_uri","meta":"collection_meta"}' --label collection --admin ${FROM_ACCOUNT}  --from ${FROM_ACCOUNT} --node ${NODE_URL} ${CHAIN_OPTION})

## confirm a result of `instantiate`
raw_log=$(echo ${result} | jq .raw_log)
judge_result_and_output "instantiate" "${raw_log}"

echo "##### TEST: issue_nft collection contract #####" >&2
CONTRACT_ADDRESS=$(echo "${result}" | jq -r '.logs[] | select(.msg_index == 0) | .events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')

result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"issue_nft":{"name":"nft1_name","meta":"nft1_meta","owner":"'${CONTRACT_ADDRESS}'"}}' --from ${FROM_ACCOUNT} --node ${NODE_URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `issue_nft`
raw_log=$(echo ${result} | jq .raw_log)
judge_result_and_output "execute: issue_nft" "${raw_log}"

echo "##### TEST: mint_nft collection contract #####" >&2
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"mint_nft":{"from":"'${CONTRACT_ADDRESS}'","to":"'${CONTRACT_ADDRESS}'","params":[{"token_type":"10000001","name":"nft1_name1","meta":"nft1_meta1"},{"token_type":"10000001","name":"nft1_name2","meta":"nft1_meta2"}]}}' --from ${FROM_ACCOUNT} --node ${NODE_URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `mint_nft`
raw_log=$(echo ${result} | jq .raw_log)
judge_result_and_output "execute: mint_nft" "${raw_log}"

echo "##### TEST: send_nft collection contract #####" >&2
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"send_nft":{"from":"'${CONTRACT_ADDRESS}'","to":"'${CONTRACT_ADDRESS}'","token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --node ${NODE_URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `send_nft`
raw_log=$(echo ${result} | jq .raw_log)
judge_result_and_output "execute: send_nft" "${raw_log}"

echo "##### TEST: burn_nft collection contract #####" >&2
result=$(fnsad tx wasm execute ${CONTRACT_ADDRESS} '{"burn_nft":{"from":"'${CONTRACT_ADDRESS}'","token_ids":["1000000100000001"]}}' --from ${FROM_ACCOUNT} --node ${NODE_URL} --gas 10000000 ${CHAIN_OPTION})

## confirm a result of `burn_nft`
raw_log=$(echo ${result} | jq .raw_log)
judge_result_and_output "execute: burn_nft" "${raw_log}"
