#![no_std]
pub mod siphash;

#[cfg(feature = "rng")]
pub mod rng;

use core::hash::BuildHasher;

pub use siphash::sys::SipHashState;
pub use siphash::RawSipHasher;
pub use siphash::SipHasher;

pub struct BuildSipHasher<const C: usize, const D: usize> {
    k0: u64,
    k1: u64,
}

impl<const C: usize, const D: usize> BuildSipHasher<C, D> {
    pub const fn new_with_keys(k0: u64, k1: u64) -> Self {
        Self { k0, k1 }
    }
}

impl<const C: usize, const D: usize> BuildHasher for BuildSipHasher<C, D> {
    type Hasher = SipHasher<C, D>;
    fn build_hasher(&self) -> Self::Hasher {
        SipHasher::new_with_keys(self.k0, self.k1)
    }
}

#[cfg(feature = "random-state")]
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
