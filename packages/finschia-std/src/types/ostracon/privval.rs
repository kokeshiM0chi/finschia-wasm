use finschia_std_derive::CosmwasmExt;
/// VRFProofRequest is a PrivValidatorSocket message containing a message to generate proof.
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
#[proto_message(type_url = "/ostracon.privval.VRFProofRequest")]
pub struct VrfProofRequest {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// VRFProofResponse is a PrivValidatorSocket message containing a Proof.
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
#[proto_message(type_url = "/ostracon.privval.VRFProofResponse")]
pub struct VrfProofResponse {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::tendermint::privval::RemoteSignerError>,
}
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
#[proto_message(type_url = "/ostracon.privval.Message")]
pub struct Message {
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5, 6, 7, 8, 1000, 1001")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    use finschia_std_derive::CosmwasmExt;
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
        PubKeyRequest(super::super::super::tendermint::privval::PubKeyRequest),
        #[prost(message, tag = "2")]
        PubKeyResponse(super::super::super::tendermint::privval::PubKeyResponse),
        #[prost(message, tag = "3")]
        SignVoteRequest(super::super::super::tendermint::privval::SignVoteRequest),
        #[prost(message, tag = "4")]
        SignedVoteResponse(super::super::super::tendermint::privval::SignedVoteResponse),
        #[prost(message, tag = "5")]
        SignProposalRequest(super::super::super::tendermint::privval::SignProposalRequest),
        #[prost(message, tag = "6")]
        SignedProposalResponse(super::super::super::tendermint::privval::SignedProposalResponse),
        #[prost(message, tag = "7")]
        PingRequest(super::super::super::tendermint::privval::PingRequest),
        #[prost(message, tag = "8")]
        PingResponse(super::super::super::tendermint::privval::PingResponse),
        #[prost(message, tag = "1000")]
        VrfProofRequest(super::VrfProofRequest),
        #[prost(message, tag = "1001")]
        VrfProofResponse(super::VrfProofResponse),
    }
}
