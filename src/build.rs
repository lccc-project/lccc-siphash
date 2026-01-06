use core::hash::BuildHasher;

use crate::{BuildSipHasher, SipHasher};

pub struct RandomState<const C: usize, const D: usize>(BuildSipHasher<C, D>);

impl<const C: usize, const D: usize> RandomState<C, D> {
    pub fn new() -> Self {
        let mut bytes = [[0u8; 8]; 2];

        getrandom::fill(bytes.as_flattened_mut()).unwrap();

        let [k0, k1] = bytes;

        Self(BuildSipHasher::new_with_keys(
            u64::from_ne_bytes(k0),
            u64::from_ne_bytes(k1),
        ))
    }
}

impl<const C: usize, const D: usize> BuildHasher for RandomState<C, D> {
    type Hasher = SipHasher<C, D>;

    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }
}
