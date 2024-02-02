use finschia_std_derive::CosmwasmExt;
/// remotesignererror is returned when the remote signer fails.
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
#[proto_message(type_url = "/cometbft.privval.v1.RemoteSignerError")]
pub struct RemoteSignerError {
    #[prost(int32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// PubKeyRequest requests the consensus public key from the remote signer.
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
#[proto_message(type_url = "/cometbft.privval.v1.PubKeyRequest")]
pub struct PubKeyRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// PubKeyResponse is a response message containing the public key.
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
#[proto_message(type_url = "/cometbft.privval.v1.PubKeyResponse")]
pub struct PubKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignVoteRequest is a request to sign a vote
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
#[proto_message(type_url = "/cometbft.privval.v1.SignVoteRequest")]
pub struct SignVoteRequest {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<super::super::types::v1::Vote>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedVoteResponse is a response containing a signed vote or an error
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
#[proto_message(type_url = "/cometbft.privval.v1.SignedVoteResponse")]
pub struct SignedVoteResponse {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<super::super::types::v1::Vote>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignProposalRequest is a request to sign a proposal
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
#[proto_message(type_url = "/cometbft.privval.v1.SignProposalRequest")]
pub struct SignProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<super::super::types::v1::Proposal>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedProposalResponse is response containing a signed proposal or an error
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
#[proto_message(type_url = "/cometbft.privval.v1.SignedProposalResponse")]
pub struct SignedProposalResponse {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<super::super::types::v1::Proposal>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// PingRequest is a request to confirm that the connection is alive.
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
#[proto_message(type_url = "/cometbft.privval.v1.PingRequest")]
pub struct PingRequest {}
/// PingResponse is a response to confirm that the connection is alive.
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
#[proto_message(type_url = "/cometbft.privval.v1.PingResponse")]
pub struct PingResponse {}
/// Message is an abstract message to/from the remote signer.
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
#[proto_message(type_url = "/cometbft.privval.v1.Message")]
pub struct Message {
    /// Sum of all possible messages.
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
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
        PubKeyRequest(super::PubKeyRequest),
        #[prost(message, tag = "2")]
        PubKeyResponse(super::PubKeyResponse),
        #[prost(message, tag = "3")]
        SignVoteRequest(super::SignVoteRequest),
        #[prost(message, tag = "4")]
        SignedVoteResponse(super::SignedVoteResponse),
        #[prost(message, tag = "5")]
        SignProposalRequest(super::SignProposalRequest),
        #[prost(message, tag = "6")]
        SignedProposalResponse(super::SignedProposalResponse),
        #[prost(message, tag = "7")]
        PingRequest(super::PingRequest),
        #[prost(message, tag = "8")]
        PingResponse(super::PingResponse),
    }
}
