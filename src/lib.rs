//! yubihsm.rs: client for `YubiHSM2` hardware security modules
//!
//! # Build Notes
//!
//! This crate depends on the `aesni` crate, which uses the "stdsimd"
//! API to invoke hardware AES instructions via `core::arch`.
//!
//! To access these features, you will need both a relatively recent
//! Rust nightly and to pass the following as RUSTFLAGS:
//!
//! `RUSTFLAGS=-Ctarget-feature=+aes`
//!
//! You can configure your `~/.cargo/config` to always pass these flags:
//!
//! ```toml
//! [build]
//! rustflags = ["-Ctarget-feature=+aes"]
//! ```

#![crate_name = "yubihsm"]
#![crate_type = "rlib"]
#![cfg_attr(feature = "bench", feature(test))]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/yubihsm/0.3.0")]

extern crate aesni;
#[macro_use]
extern crate bitflags;
extern crate block_modes;
extern crate byteorder;
extern crate clear_on_drop;
extern crate cmac;
extern crate constant_time_eq;
#[cfg(feature = "dalek")]
extern crate ed25519_dalek;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate hmac;
extern crate pbkdf2;
extern crate rand;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sha2;
#[cfg(feature = "bench")]
extern crate test;

#[macro_use]
mod macros;

pub mod algorithm;
#[cfg(feature = "bench")]
mod bench;
pub mod capabilities;
mod commands;
pub mod connector;
pub mod domains;
#[cfg(feature = "mockhsm")]
pub mod mockhsm;
pub mod object;
pub mod responses;
mod securechannel;
mod serializers;
pub mod session;

pub use algorithm::Algorithm;
pub use capabilities::Capabilities;
pub use connector::Connector;
pub use domains::Domains;
pub use object::Id as ObjectId;
pub use object::Label as ObjectLabel;
pub use object::Origin as ObjectOrigin;
pub use object::Type as ObjectType;
pub use object::SequenceId;
pub use securechannel::SessionId;
pub use session::{Session, SessionError};
