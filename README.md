> :information_desk_person: If you want to use latest update from finschia' main branch, checkout `autobuild-main` branch.

This repository is forked from [osmosis-labs/osmosis-rust](https://github.com/osmosis-labs/osmosis-rust)

# finschia-std

Rust libraries for Finschia. The following table shows every published crates maintained in this repository:

| Crate                                             | Description                                                                                                                                                            | Crates.io                                                                                                                                 | Docs                                                                                        |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| [finschia-std](packages/finschia-std)               | Finschia's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.                                                      | [![finschia-std on crates.io](https://img.shields.io/crates/v/finschia-std.svg)](https://crates.io/crates/finschia-std)                      | [![Docs](https://docs.rs/finschia-std/badge.svg)](https://docs.rs/finschia-std)               |
| [finschia-std-derive](packages/finschia-std-derive) | Procedural macro for augmenting proto-generated types to create better developer ergonomics. Internally used by `finschia-std`                                          | [![finschia-std-derive on crates.io](https://img.shields.io/crates/v/finschia-std-derive.svg)](https://crates.io/crates/finschia-std-derive) | [![Docs](https://docs.rs/finschia-std-derive/badge.svg)](https://docs.rs/finschia-std-derive) |
---

This repo also contains [`proto-build`](./packages/proto-build) package which is used for autogenrating rust types from proto.
