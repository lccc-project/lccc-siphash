//! Module that provides the [`RandomState`] type.

use core::hash::BuildHasher;

use crate::{BuildSipHasher, SipHasher};

/// [`RandomState`] is a [`BuildHasher`] that yields the [`SipHasher`] type. Rather than being constructed from fixed keys,
/// it has a single constructor [`RandomState::new`], which generates random keys.
/// The resulting [`RandomState`] will then produce equal [`SipHasher`] instances for each [`BuildHasher::build_hasher`] call, but different [`RandomState`] instances will have different values.
///
/// `C` and `D` are the parameters of SipHash-*C*-*D* for the returned [`SipHasher`] instance
#[derive(Clone, Debug)]
pub struct RandomState<const C: usize, const D: usize>(BuildSipHasher<C, D>);

impl<const C: usize, const D: usize> Default for RandomState<C, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const C: usize, const D: usize> RandomState<C, D> {
    /// Constructs a new, unique [`RandomState`] instance.
    ///
    /// The precise distribution of values is not specified and depends on the quality of the underlying random number generator, however on most platforms,
    ///  it should take, on average, 2^64 calls to this function to have a 50% chance of any two instances using identical keys.
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
