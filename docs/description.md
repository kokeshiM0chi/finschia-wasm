# What you can learn from this document

This document allows you to understand what it's doing without reading Source code.

## What is finschia-wasm?

This is the feature to call the finchia-sdk module from "contract".
This feature is motivated by the fact that osmosis-rust has a library for calling modules with contracts.

For this reason, we are forking osmosis-rust.
https://github.com/osmosis-labs/osmosis-rust

## How does finschia-wasm works?

I will give an overview of this repository.
repository URL: https://github.com/Finschia/finschia-wasm

This chapter provides an overview of this repository and presents its architecture.
I will explain how to use it just by looking at it, without having to read the source code.

## Summary

This library consists of the following three packages.

- fischia-std
- finschia-std-derive
- proto-build

Each package is explained below.

## `finschia-std`

You can find all types and queries generated from finschia's protobuf in their respective module in `finschia_std`.

To know how each module works, see the list below.

### Module list

- [finschia-sdk](https://github.com/Finschia/finschia-sdk/tree/v0.49.1/proto)
- [ostracon](https://github.com/Finschia/ostracon/tree/v1.1.5/proto/ostracon)
- [wasmd](https://github.com/Finschia/wasmd/tree/v0.51.0/proto)

## Directory structure

```
./packages
├── finschia-std
├── finschia-std-derive
└── proto-build
```

### proto-build

This process compiles the specified protofiles downloaded to local and generates a module tree as rust.
The generated module tree will be output as a rust module directly under finschia-std, which will be introduced next.

### finschia-std

This package has the following structure.

```shell
src
├── lib.rs
├── serde
├── shim.rs
└── types
```

All types generated from finschia's protbuf can be found under the `types` directory.

I will explain this module path rule using the following protofile "collection.proto" as an example.
When calling from a contract, specify the path and import the type you want to use according to this rule.
Here's an example.
[collection.proto](https://github.com/Finschia/finschia-sdk/blob/v0.49.1/proto/lbm/collection/v1/collection.proto
)

#### `types` path rule

Look at this second line of this protofile.
https://github.com/Finschia/finschia-sdk/blob/v0.49.1/proto/lbm/collection/v1/collection.proto#L2

```
package lbm.collection.v1;
```

Package names are separated by ".". This "." delimiter defines the module hierarchy.
Therefore, the above package name will be placed in the following path.
https://github.com/Finschia/finschia-wasm/blob/v2.0.1/packages/finschia-std/src/types/lbm/collection/v1.rs

#### How to import type according to the above path rule

When calling with a contract, call it like this.
For example, use `MsgMintNft` in a function that mints message.

```rust
use finschia_std::types::lbm::collection::v1::MsgMintNft

pub fn try_mint_nft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    params: Vec<MintNftParam>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_mint_nft: CosmosMsg = MsgMintNft {
        contract_id,
        from,
        to,
        params,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_mint_nft")
        .add_submessage(SubMsg::reply_on_success(msg_mint_nft, MINT_NFT_REPLY_ID)))
}
```

When rust calls, `finschia-std` is converted to `finschia_std`.
This is because according to the Rust language specification, `-` is converted to `_` and interpreted.

### finschia-std-derive

Look at the example above. `MsgMintNft` has been converted to `CosmosMsg` type. 
This package provides this type conversion using macros.


## Execution

Download protofiles.

```rust
git submodule --init update
```

Compile protofiles.

```rust
cd packages/proto-build
```

Generate rust module

```rust
cargo run
```

## Support module

Registered in git submodule.

```shell
finschia-std % git submodule status
-acd1ec626935e9861a124c4d03271214dc052fdf ../../dependencies/finschia-sdk
-dd93e7fcdfdade7c567d3dc1a04b373fc98fbaeb ../../dependencies/ibc-go
-74ce807b7be39a7e0afb4e2efb8e28a57965f57b ../../dependencies/ics23
-08c34d4f52caa73faacabcab092b6ad65a35e6c2 ../../dependencies/ostracon
-014cdcf09844d48f6d30f3e520034b7edffd9670 ../../dependencies/tendermint
-26de8ffab7bd1a7dbb0ca4526b42411c8d749d81 ../../dependencies/wasmd
```

## Version management

### About env file

This env file manages the version of each module to be compiled.
https://github.com/Finschia/finschia-wasm/blob/main/env

## GitHub action

Download the finschia's proto of each repository based on the above env file version.
This version control method follows the same rules as [finschia-proto](https://github.com/Finschia/finschia-proto).
