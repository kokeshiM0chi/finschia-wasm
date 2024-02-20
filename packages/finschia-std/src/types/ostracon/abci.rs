use finschia_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/ostracon.abci.Request")]
pub struct Request {
    #[prost(
        oneof = "request::Value",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1000, 1001"
    )]
    pub value: ::core::option::Option<request::Value>,
}
/// Nested message and enum types in `Request`.
pub mod request {
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
    pub enum Value {
        #[prost(message, tag = "1")]
        Echo(super::super::super::tendermint::abci::RequestEcho),
        #[prost(message, tag = "2")]
        Flush(super::super::super::tendermint::abci::RequestFlush),
        #[prost(message, tag = "3")]
        Info(super::super::super::tendermint::abci::RequestInfo),
        #[prost(message, tag = "4")]
        SetOption(super::super::super::tendermint::abci::RequestSetOption),
        #[prost(message, tag = "5")]
        InitChain(super::super::super::tendermint::abci::RequestInitChain),
        #[prost(message, tag = "6")]
        Query(super::super::super::tendermint::abci::RequestQuery),
        #[prost(message, tag = "7")]
        BeginBlock(super::RequestBeginBlock),
        #[prost(message, tag = "8")]
        CheckTx(super::super::super::tendermint::abci::RequestCheckTx),
        #[prost(message, tag = "9")]
        DeliverTx(super::super::super::tendermint::abci::RequestDeliverTx),
        #[prost(message, tag = "10")]
        EndBlock(super::super::super::tendermint::abci::RequestEndBlock),
        #[prost(message, tag = "11")]
        Commit(super::super::super::tendermint::abci::RequestCommit),
        #[prost(message, tag = "12")]
        ListSnapshots(super::super::super::tendermint::abci::RequestListSnapshots),
        #[prost(message, tag = "13")]
        OfferSnapshot(super::super::super::tendermint::abci::RequestOfferSnapshot),
        #[prost(message, tag = "14")]
        LoadSnapshotChunk(super::super::super::tendermint::abci::RequestLoadSnapshotChunk),
        #[prost(message, tag = "15")]
        ApplySnapshotChunk(super::super::super::tendermint::abci::RequestApplySnapshotChunk),
        /// 16~99 are reserved for merging original tendermint
        #[prost(message, tag = "1000")]
        BeginRecheckTx(super::RequestBeginRecheckTx),
        #[prost(message, tag = "1001")]
        EndRecheckTx(super::RequestEndRecheckTx),
    }
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
#[proto_message(type_url = "/ostracon.abci.RequestBeginBlock")]
pub struct RequestBeginBlock {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<super::super::tendermint::types::Header>,
    #[prost(message, optional, tag = "3")]
    pub last_commit_info: ::core::option::Option<super::super::tendermint::abci::LastCommitInfo>,
    #[prost(message, repeated, tag = "4")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<super::super::tendermint::abci::Evidence>,
    /// *** Ostracon Extended Fields ***
    #[prost(message, optional, tag = "1000")]
    pub entropy: ::core::option::Option<super::types::Entropy>,
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
#[proto_message(type_url = "/ostracon.abci.RequestBeginRecheckTx")]
pub struct RequestBeginRecheckTx {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::tendermint::types::Header>,
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
#[proto_message(type_url = "/ostracon.abci.RequestEndRecheckTx")]
pub struct RequestEndRecheckTx {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
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
#[proto_message(type_url = "/ostracon.abci.Response")]
pub struct Response {
    #[prost(
        oneof = "response::Value",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1000, 1001"
    )]
    pub value: ::core::option::Option<response::Value>,
}
/// Nested message and enum types in `Response`.
pub mod response {
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
    pub enum Value {
        #[prost(message, tag = "1")]
        Exception(super::super::super::tendermint::abci::ResponseException),
        #[prost(message, tag = "2")]
        Echo(super::super::super::tendermint::abci::ResponseEcho),
        #[prost(message, tag = "3")]
        Flush(super::super::super::tendermint::abci::ResponseFlush),
        #[prost(message, tag = "4")]
        Info(super::super::super::tendermint::abci::ResponseInfo),
        #[prost(message, tag = "5")]
        SetOption(super::super::super::tendermint::abci::ResponseSetOption),
        #[prost(message, tag = "6")]
        InitChain(super::super::super::tendermint::abci::ResponseInitChain),
        #[prost(message, tag = "7")]
        Query(super::super::super::tendermint::abci::ResponseQuery),
        #[prost(message, tag = "8")]
        BeginBlock(super::super::super::tendermint::abci::ResponseBeginBlock),
        #[prost(message, tag = "9")]
        CheckTx(super::ResponseCheckTx),
        #[prost(message, tag = "10")]
        DeliverTx(super::super::super::tendermint::abci::ResponseDeliverTx),
        #[prost(message, tag = "11")]
        EndBlock(super::super::super::tendermint::abci::ResponseEndBlock),
        #[prost(message, tag = "12")]
        Commit(super::super::super::tendermint::abci::ResponseCommit),
        #[prost(message, tag = "13")]
        ListSnapshots(super::super::super::tendermint::abci::ResponseListSnapshots),
        #[prost(message, tag = "14")]
        OfferSnapshot(super::super::super::tendermint::abci::ResponseOfferSnapshot),
        #[prost(message, tag = "15")]
        LoadSnapshotChunk(super::super::super::tendermint::abci::ResponseLoadSnapshotChunk),
        #[prost(message, tag = "16")]
        ApplySnapshotChunk(super::super::super::tendermint::abci::ResponseApplySnapshotChunk),
        /// 17~99 are reserved for merging original tendermint
        #[prost(message, tag = "1000")]
        BeginRecheckTx(super::ResponseBeginRecheckTx),
        #[prost(message, tag = "1001")]
        EndRecheckTx(super::ResponseEndRecheckTx),
    }
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
#[proto_message(type_url = "/ostracon.abci.ResponseCheckTx")]
pub struct ResponseCheckTx {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_used: i64,
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<super::super::tendermint::abci::Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
    /// MEMO: not used, just reservation to implement <https://github.com/tendermint/tendermint/pull/6740> first
    #[prost(string, tag = "9")]
    pub sender: ::prost::alloc::string::String,
    /// MEMO: not used, just reservation to implement <https://github.com/tendermint/tendermint/pull/6740> first
    #[prost(int64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub priority: i64,
    /// mempool_error is set by Ostracon.
    /// ABCI applictions creating a ResponseCheckTX should not set mempool_error.
    #[prost(string, tag = "11")]
    pub mempool_error: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/ostracon.abci.ResponseBeginRecheckTx")]
pub struct ResponseBeginRecheckTx {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code: u32,
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
#[proto_message(type_url = "/ostracon.abci.ResponseEndRecheckTx")]
pub struct ResponseEndRecheckTx {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code: u32,
}
