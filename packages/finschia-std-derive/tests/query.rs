use cosmwasm_std::{Empty, QueryRequest};
use finschia_std_derive::CosmwasmExt;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.v1beta1.Coin")]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}

#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryBalanceRequest")]
#[proto_query(path = "/cosmos.bank.v1beta1.Query/Balance", response_type = QueryBalanceResponse)]
pub struct QueryBalanceRequest {
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}

#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// balance is the balance of the coin.
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<Coin>,
}

fn main() {
    let _: QueryRequest<Empty> = QueryBalanceRequest {
        address: "link15la35q37j2dcg427kfy4el2l0r227xwhuaapxd".to_string(),
        denom: "link".to_string(),
    }
    .into();
}

mod shim {
    pub struct Any {
        pub type_url: String,
        pub value: Vec<u8>,
    }
}
