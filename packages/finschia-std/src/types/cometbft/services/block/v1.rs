use finschia_std_derive::CosmwasmExt;
/// GetByHeightRequest is a request for a block at the specified height.
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
#[proto_message(type_url = "/cometbft.services.block.v1.GetByHeightRequest")]
pub struct GetByHeightRequest {
    /// The height of the block requested.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
}
/// GetByHeightResponse contains the block ID and the block at the specified height.
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
#[proto_message(type_url = "/cometbft.services.block.v1.GetByHeightResponse")]
pub struct GetByHeightResponse {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<super::super::super::types::v1::BlockId>,
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<super::super::super::types::v1::Block>,
}
/// GetLatestHeightRequest - empty message since no parameter is required
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
#[proto_message(type_url = "/cometbft.services.block.v1.GetLatestHeightRequest")]
pub struct GetLatestHeightRequest {}
/// GetLatestHeightResponse provides the height of the latest committed block.
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
#[proto_message(type_url = "/cometbft.services.block.v1.GetLatestHeightResponse")]
pub struct GetLatestHeightResponse {
    /// The height of the latest committed block. Will be 0 if no data has been
    /// committed yet.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
}
