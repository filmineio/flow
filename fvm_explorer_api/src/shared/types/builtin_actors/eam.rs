use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::{Cbor, RawBytes};
use fvm_shared::address::Address;

#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct CreateParams {
    pub initcode: RawBytes,
    pub nonce: u64,
}

#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct Create2Params {
    pub initcode: RawBytes,
    pub salt: RawBytes,
}

#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct CreateAccount {
    pub pubkey: RawBytes,
}

impl Cbor for CreateParams {}
impl Cbor for Create2Params {}
impl Cbor for CreateAccount {}

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EAMReturn {
    pub actor_id: u64,
    pub robust_address: Address,
    pub eth_address: RawBytes,
}

impl Cbor for EAMReturn {}
