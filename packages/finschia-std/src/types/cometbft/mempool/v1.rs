use finschia_std_derive::CosmwasmExt;
/// Txs contains a list of transaction from the mempool.
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
#[proto_message(type_url = "/cometbft.mempool.v1.Txs")]
pub struct Txs {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Message is an abstract mempool message.
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
#[proto_message(type_url = "/cometbft.mempool.v1.Message")]
pub struct Message {
    /// Sum of all possible messages.
    #[prost(oneof = "message::Sum", tags = "1")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    use finschia_std_derive::CosmwasmExt;
    /// Sum of all possible messages.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Sum {
        #[prost(message, tag = "1")]
        Txs(super::Txs),
    }
}
