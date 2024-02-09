use finschia_std_derive::CosmwasmExt;
/// GetBlockResults is a request for the BlockResults of a given height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cometbft.services.block_results.v1.GetBlockResultsRequest")]
pub struct GetBlockResultsRequest {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
}
/// GetBlockResultsResponse contains the block results for the given height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cometbft.services.block_results.v1.GetBlockResultsResponse")]
pub struct GetBlockResultsResponse {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(message, repeated, tag = "2")]
    pub tx_results: ::prost::alloc::vec::Vec<super::super::super::abci::v1::ExecTxResult>,
    #[prost(message, repeated, tag = "3")]
    pub finalize_block_events: ::prost::alloc::vec::Vec<super::super::super::abci::v1::Event>,
    #[prost(message, repeated, tag = "4")]
    pub validator_updates: ::prost::alloc::vec::Vec<super::super::super::abci::v1::ValidatorUpdate>,
    #[prost(message, optional, tag = "5")]
    pub consensus_param_updates:
        ::core::option::Option<super::super::super::types::v1::ConsensusParams>,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
