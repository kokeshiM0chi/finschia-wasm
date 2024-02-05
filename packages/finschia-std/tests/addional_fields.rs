use cosmwasm_std::{from_binary, to_binary};
use finschia_std_derive::CosmwasmExt;
use prost::Message;

#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.foundation.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/lbm.foundation.v1.Params")]
pub struct Params {
    #[prost(string, tag = "1")]
    pub foundation_tax: ::prost::alloc::string::String,
}

#[test]
fn test_additional_fields_does_not_break_but_cause_lossy_json_deserialization() {
    let response = QueryParamsResponse {
        params: Some(Params {
            foundation_tax: "foundation tax sample".to_string(),
        }),
    };

    // to_binary() and from_binary() is using `serde_json_wasm` under the hood.
    let serialized = to_binary(&response.params.unwrap()).unwrap();
    let deserialized: Params = from_binary(&serialized).unwrap();

    // lossy deserialization
    assert_eq!(
        deserialized,
        Params {
            foundation_tax: "foundation tax sample".to_string()
        }
    );
}

#[test]
fn test_additional_fields_does_not_break_but_cause_lossy_proto_deserialization() {
    let response = QueryParamsResponse {
        params: Some(Params {
            foundation_tax: "foundation tax sample".to_string(),
        }),
    };

    let serialized = response.params.unwrap().encode_to_vec();
    let deserialized = Params::decode(&serialized[..]).unwrap();

    // lossy deserialization
    assert_eq!(
        deserialized,
        Params {
            foundation_tax: "foundation tax sample".to_string()
        }
    );
}

mod shim {
    pub struct Any {
        pub type_url: String,
        pub value: Vec<u8>,
    }
}
