use cosmwasm_std::CosmosMsg;
use finschia_std_derive::CosmwasmExt;

#[derive(Clone, PartialEq, Eq, ::prost::Message, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.bank.v1beta1.SendEnabled")]
pub struct SendEnabled {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}

fn main() {
    assert_eq!(SendEnabled::TYPE_URL, "/cosmos.bank.v1beta1.SendEnabled");
    let msg = SendEnabled {
        denom: "uxxx".to_string(),
        enabled: true,
    };

    let _: CosmosMsg = msg.into();
}

mod shim {
    pub struct Any {
        pub type_url: String,
        pub value: Vec<u8>,
    }
}
