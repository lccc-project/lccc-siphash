#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![cfg_attr(feature = "nightly-docs", feature(doc_cfg))]
#![cfg_attr(
    feature = "nightly-prefixfree_extras",
    feature(hasher_prefixfree_extras)
)]

//! lccc-siphash implements the SipHash algorithm with a generic number of update and finalize rounds.
//!
//! The implementation is designed to be highly optimized, making use of low-level hardware instructions to generate the most efficient code possible.
//! This is based on the static set of supported target features and the cpu. Because of this, compilation with -C target-cpu is recommended.
//!
//! ## Features
//!
//! The following features are supported (features prefixed with `nightly-` require an up-to-date nightly compiler and are not considered part of the semver API):
//! * `inspect-raw`: Allows extraction of the raw [`SipHashState`] from hashers and random generators
//! * `rng`: Adds the type [`rng::SiphashRng`], to generate random numbers using the siphash impl
//! * `rand_core`: Adds the optional `rand_core` dependency and implements it for [`rng::SiphashRng`]
//! * `serde`: Adds serde support for serializing and deserializing raw states.
//! * `random-state`: Adds the type [`build::RandomState`] as a [`BuildHasher`] impl. This adds a dependency on the `getrandom` crate.
//! * `nightly-prefixfree_extras`: Implements [`Hasher::write_str`][core::hash::Hasher::write_str] in an optimized way. Note that this changes the results of hashes that involve `str` or `String`.
//!
//! ## [`RandomState`][build::RandomState] and wasm
//!
//! The [`RandomState`][build::RandomState] type allows using a highly collision-resistant generator for random.
//! This is implemented using the `getrandom` crate. In order to support this on wasm (and certain bare metal targets), you must provide a getrandom provider.
//! On web, you can do this by depending directly on `getrandom` 0.3 (or later) and enabling the `wasm_js` feature.

#[allow(unexpected_cfgs)]
pub mod siphash;

#[cfg(any(doc, feature = "rng"))]
pub mod rng;

use core::hash::BuildHasher;

#[cfg(feature = "rand_core")]
use rand_core::{Rng, TryRng};
pub use siphash::RawSipHasher;
pub use siphash::SipHashState;
pub use siphash::SipHasher;

/// Default [`BuildHasher`] for [`SipHasher`]. `C` and `D` are the configuration parameters for SipHash-*C*-*D*, specifying the number of update rounds (C) and finalization rounds (D).
#[derive(Clone, Debug)]
pub struct BuildSipHasher<const C: usize, const D: usize> {
    k0: u64,
    k1: u64,
}

impl<const C: usize, const D: usize> BuildSipHasher<C, D> {
    /// Constructs a new [`BuildSipHasher`] with the specified set of keys. All [`BuildSipHasher`] instances constructed with the same keys will produce identical hashers.
    pub const fn new_with_keys(k0: u64, k1: u64) -> Self {
        Self { k0, k1 }
    }

    /// Constructs a new [`BuildSipHasher`] with keys populated from the specified [`Rng`].
    /// If the Rng being used is the system rng, it may be better to use [`RandomState`][build::RandomState] instead (and enable the `random_state` feature)
    #[cfg(feature = "rand_core")]
    pub fn from_rng<R: Rng>(r: &mut R) -> Self {
        let k0 = r.next_u64();
        let k1 = r.next_u64();

        Self::new_with_keys(k0, k1)
    }

    /// Constructs a new [`BuildSipHasher`] with keys populated from the specified [`TryRng`], failing if an error occurs
    /// If the Rng being used is the system rng, it may be better to use [`RandomState`][build::RandomState] instead (and enable the `random_state` feature)
    #[cfg(feature = "rand_core")]
    pub fn try_from_rng<R: TryRng>(r: &mut R) -> Result<Self, R::Error> {
        let k0 = r.try_next_u64()?;
        let k1 = r.try_next_u64()?;

        Ok(Self::new_with_keys(k0, k1))
    }
}

impl<const C: usize, const D: usize> BuildHasher for BuildSipHasher<C, D> {
    type Hasher = SipHasher<C, D>;
    fn build_hasher(&self) -> Self::Hasher {
        SipHasher::new_with_keys(self.k0, self.k1)
    }
}

#[cfg(any(doc, feature = "random-state"))]
pub mod build;

#[cfg(test)]
mod test {
    use crate::SipHasher;
    use core::hash::Hasher;

    pub struct TestVector {
        k0: u64,
        k1: u64,
        data: &'static [u8],
        expected: u64,
    }

    pub const SIPHASH_2_4_TEST_VECTORS: &[TestVector] = &[TestVector {
        k0: 0x0706050403020100,
        k1: 0x0f0e0d0c0b0a0908,
        data: &[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E,
        ],
        expected: 0xa129ca6149be45e5,
    }];

    #[test]
    pub fn siphash_2_4_tests() {
        for vec in SIPHASH_2_4_TEST_VECTORS {
            let mut hasher = SipHasher::<2, 4>::new_with_keys(vec.k0, vec.k1);
            hasher.write(vec.data);
            let got = hasher.finish();
            assert_eq!(vec.expected, got, "{:#016x}!={:#016x}", vec.expected, got);
        }
    }
}
