# finschia-std

Finschia's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.

## CosmWasm stargate message and stargate query

You can find all types and querier generated from finschia's protobuf in their respective module in `finschia_std`. To understand how each module works, please look at the [finschia-sdk modules](https://github.com/Finschia/finschia-sdk/blob/main/x/README.md).

[Full working example contract can be found here.](https://github.com/Finschia/finschia-wasm/tree/main/examples/contracts/collection)

### Publishing Finschia' message from CosmWasm Contract

```rust
use cosmwasm_std::{
    CosmosMsg, DepsMut
};
use finschia_std::types::lbm::collection::v1::MsgIssueNft;

# type ContractError = cosmwasm_std::StdError;
// ..

pub fn try_issue_nft(
    deps: DepsMut,
    name: String,
    meta: String,
    owner: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_issue_nft: CosmosMsg = MsgIssueNft {
        contract_id,
        name,
        meta,
        owner,
    }
        .into();

    Ok(Response::new()
        .add_attribute("method", "try_issue_nft")
        .add_submessage(SubMsg::reply_on_success(msg_issue_nft, ISSUE_NFT_REPLY_ID)))
}

```


## Non-CosmWasm Client

(WIP)
