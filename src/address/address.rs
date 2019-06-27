// Rust Bitcoin Library
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//


use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::network::Network;
use crate::script::{Script,op_codes};
use crate::util::{key,base58,hash160,hash256};

/// The method used to produce an address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Payload {
    /// pay-to-pkhash address
    PubkeyHash(hash160::Hash160),
    /// P2SH address
    ScriptHash(hash160::Hash160),
}

#[derive(Clone, PartialEq, Eq, Hash)]
/// A Bitcoin address
pub struct Address {
    /// The type of the address
    pub payload: Payload,
    /// The network on which this address is usable
    pub network: Network,
}

impl Address {
    /// Creates a pay to (compressed) public key hash address from a public key
    /// This is the preferred non-witness type address
    #[inline]
    pub fn p2pkh(pk: &key::PublicKey, network: Network) -> Address {
        Address {
            network,
            payload: Payload::PubkeyHash(hash160::hash160(&pk.to_bytes()))
        }
    }

    /// Creates a pay to script hash P2SH address from a script
    /// This address type was introduced with BIP16 and is the popular type to implement multi-sig these days.
    #[inline]
    pub fn p2sh(script: &Script, network: Network) -> Address {
        Address {
            network,
            payload: Payload::ScriptHash(hash160::hash160(&script.0))
        }
    }


//    /// Create a pay to script address that embeds a witness pay to public key
//    /// This is a segwit address type that looks familiar (as p2sh) to legacy clients
//    pub fn p2shwpkh (pk: &key::PublicKey, network: Network) -> Address {
//        let mut hash_engine = hash160::Hash::engine();
//        pk.write_into(&mut hash_engine);
//
//        let builder = script::Builder::new()
//            .push_int(0)
//            .push_slice(&hash160::Hash::from_engine(hash_engine)[..]);
//
//        Address {
//            network,
//            payload: Payload::ScriptHash(
//                hash160::Hash::hash(builder.into_script().as_bytes())
//            )
//        }
//    }
//
//    /// Create a pay to script address that embeds a witness pay to script hash address
//    /// This is a segwit address type that looks familiar (as p2sh) to legacy clients
//    pub fn p2shwsh (script: &Script, network: Network) -> Address {
//        use bitcoin_hashes::sha256;
//        use bitcoin_hashes::Hash;
//        use bitcoin_hashes::hash160;
//
//        let ws = script::Builder::new().push_int(0)
//                                       .push_slice(&sha256::Hash::hash(&script[..])[..])
//                                       .into_script();
//
//        Address {
//            network,
//            payload: Payload::ScriptHash(hash160::Hash::hash(&ws[..]))
//        }
//    }

}

impl Display for Address {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.payload {
            Payload::PubkeyHash(ref hash) => {
                let mut prefixed = [0; 21];
                prefixed[0] = match self.network {
                    Network::Mainnet => 38,
                    Network::Testnet => 111,
                };
                prefixed[1..].copy_from_slice(&hash.0[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            },
            Payload::ScriptHash(ref hash) => {
                let mut prefixed = [0; 21];
                prefixed[0] = match self.network {
                    Network::Mainnet => 23,
                    Network::Testnet => 196,
                };
                prefixed[1..].copy_from_slice(&hash.0[..]);
                base58::check_encode_slice_to_fmt(fmt, &prefixed[..])
            },
        }
    }
}

impl ::std::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Address {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt::{self, Formatter};

        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a Bitcoin address")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Address::from_str(v).map_err(E::custom)
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(v)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

