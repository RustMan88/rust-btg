//! A foundation for building applications on Bitcoin SV using Rust.

extern crate byteorder;
extern crate digest;
extern crate dns_lookup;
extern crate hex;
#[macro_use]
extern crate log;
extern crate linked_hash_map;
extern crate murmur3;
extern crate rand;
extern crate ring;
extern crate ripemd160;
extern crate rust_base58;
extern crate secp256k1;
extern crate snowflake;

pub mod address;
pub mod messages;
pub mod network;
pub mod peer;
pub mod script;
pub mod transaction;
pub mod util;
pub mod wallet;


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    GreateRawTxError,
    NotFoundKeyError,
    SignRawTxError,
    NotSupportedAddressFormError,
    TxidParseError,
    AddressParseError,
    PrivKeyParseError,
    NotEnoughAmount,
    PrepareRawTxError,
    NotFoundAesKeyError,
    AesDecryptError,
    SerdeJsonError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInputReq {
    pub txid: String,
    pub index: u32,
    pub address: String,
    pub credit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutputReq {
    pub address: String,
    pub value: u64,
}

pub use crate::util::key::{PublicKey,PrivateKey};
pub use crate::util::account::Account;
pub use crate::address::address::Address;
pub use crate::transaction::raw;
pub use crate::network::Network;


