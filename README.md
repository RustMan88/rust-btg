# Rust-BTG

A library to build Bitcoin BTG applications in Rust.

[Documentation](https://docs.rs/btg/)

Features

* P2P protocol messages (construction and serialization)
* Address encoding and decoding
* Transaction signing
* Script evaluation
* Node connections and basic message handling
* Wallet key derivation and mnemonic parsing
* Mainnet and testnet support
* Various Bitcoin primitives

# Installation

Add ```btg = "0.1"``` to Cargo.toml

# Requirements

Rust nightly is required for documentation due to a bug fix which has not yet made it to stable.

Run ./configure once to setup nightly.

# Known limitations

This library should not be used for consensus code because its validation checks are incomplete.

# Comparison with other Rust libraries

*rust-bitcoin* - rust-btg has no ties to rust-bitcoin. This library can do everything rust-bitcoin can do and more for Bitcoin SV.

*parity-bitcoin* - The parity Bitcoin client is a full node in Rust. Its code is more full-featured and also more complex.

*bitcrust* - The bitcrust project is strong in some areas and lacking in others. The two projects could be used together.

# License

rust-btg is licensed under the MIT license.
