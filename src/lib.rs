//! Multihash implementation.
//!
//! Feature Flags
//! -------------
//!
//! Multihash has lots of [feature flags], by default all features, except for `arb`, `scale-codec`
//! and `serde-codec` are enabled.
//!
//! Some of the features are about specific hash functions, these are:
//!
//!  - `blake2b`: Enable Blake2b hashers
//!  - `blake2s`: Enable Blake2s hashers
//!  - `sha1`: Enable SHA-1 hashers
//!  - `sha2`: Enable SHA-2 hashers
//!  - `sha3`: Enable SHA-3 hashers
//!  - `strobe`: Enable Strobe hashers
//!
//! In order to enable all hashers, you can set the `all` feature flag.
//!
//! The library has support for `no_std`, if you disable the `std` feature flag.
//!
//! The `multihash-impl` feature flag enables a default Multihash implementation that contains some
//! of the bundled hashers. If you want a different set of hash algorithms or add one which isn't
//! supported by default, you will disable that feature. Intead enable the `derive` feature in
//! order to be able to use the [`Multihash` derive], together with the features for the hashers
//! you need.
//!
//! The `arb` feature flag enables the quickcheck arbitrary implementation for property based
//! testing.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [`Multihash` derive]: crate::derive

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(test, feature = "arb"))]
mod arb;
mod error;
mod hasher;
mod hasher_impl;
mod multihash;
#[cfg(feature = "multihash-impl")]
mod multihash_impl;

pub use crate::error::{Error, Result};
#[cfg(feature = "std")]
pub use crate::hasher::WriteHasher;
pub use crate::hasher::{Digest, Hasher, Size, StatefulHasher};
pub use crate::multihash::{Multihash, MultihashDigest};
pub use generic_array::typenum::{self, U128, U16, U20, U28, U32, U48, U64};
#[cfg(feature = "derive")]
pub use multihash_derive as derive;

#[cfg(feature = "multihash-impl")]
pub use crate::multihash_impl::Code;

#[cfg(feature = "blake2b")]
pub use crate::hasher_impl::blake2b::{Blake2b256, Blake2b512, Blake2bDigest, Blake2bHasher};
#[cfg(feature = "blake2s")]
pub use crate::hasher_impl::blake2s::{Blake2s128, Blake2s256, Blake2sDigest, Blake2sHasher};
#[cfg(feature = "blake3")]
pub use crate::hasher_impl::blake3::{Blake3Digest, Blake3Hasher, Blake3_256};
pub use crate::hasher_impl::identity::{Identity256, IdentityDigest, IdentityHasher};
#[cfg(feature = "sha1")]
pub use crate::hasher_impl::sha1::{Sha1, Sha1Digest};
#[cfg(feature = "sha2")]
pub use crate::hasher_impl::sha2::{Sha2Digest, Sha2_256, Sha2_512};
#[cfg(feature = "sha3")]
pub use crate::hasher_impl::sha3::{Keccak224, Keccak256, Keccak384, Keccak512, KeccakDigest};
#[cfg(feature = "sha3")]
pub use crate::hasher_impl::sha3::{Sha3Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
#[cfg(feature = "strobe")]
pub use crate::hasher_impl::strobe::{Strobe256, Strobe512, StrobeDigest, StrobeHasher};
pub use crate::hasher_impl::unknown::UnknownDigest;
