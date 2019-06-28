//! Miscellaneous helpers

use std::time::SystemTime;

mod amount;
#[allow(dead_code)]
mod bits;
mod bloom_filter;
#[allow(dead_code)]
mod future;
#[allow(dead_code)]
pub mod hash160;
pub mod hash256;
#[allow(dead_code)]
mod latch;
mod result;
pub mod rx;
mod serdes;
pub mod key;
pub mod account;
pub mod base58;
pub(crate) mod var_int;

pub use self::amount::{Amount, Units};
pub(crate) use self::bits::{lshift, rshift, Bits};
#[allow(dead_code)]
pub use self::bloom_filter::{
    BloomFilter, BLOOM_FILTER_MAX_FILTER_SIZE, BLOOM_FILTER_MAX_HASH_FUNCS,
};
pub use self::hash160::{hash160, Hash160};
pub use self::hash256::{sha256d, Hash256};
pub use self::result::{Error, Result};
#[allow(unused_imports)]
pub use self::serdes::Serializable;

/// Gets the time in seconds since a time in the past
pub fn secs_since(time: SystemTime) -> u32 {
    SystemTime::now().duration_since(time).unwrap().as_secs() as u32
}
