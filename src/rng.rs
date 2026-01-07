//! Module providing a random number generator based on [`RawSipHasher`].

use crate::RawSipHasher;

/// [`SiphashRng`] is a random number generator that uses [`RawSipHasher`] to generate a stream of high-quality pseudo-random numbers
#[derive(Clone, Debug)]
pub struct SiphashRng<const C: usize, const D: usize>(RawSipHasher<C, D>);

impl<const C: usize, const D: usize> SiphashRng<C, D> {
    /// Constructs a new [`SipHashRng`] with the specified keys.
    pub const fn new_with_keys(k0: u64, k1: u64) -> Self {
        Self(RawSipHasher::from_keys(k0, k1))
    }

    /// Constructs a new [`SipHashRng`] from a specified [`RawSipHasher`], which may have already injested arbitrary data
    pub const fn from_raw(raw: RawSipHasher<C, D>) -> Self {
        Self(raw)
    }

    /// Convience function for constructing a [`SiphashRng`] from a single word seed. Note that this function produces a generator with a maximum enthropy of 64 bits.
    /// This function should be preferred compared to simply passing `seed` into either `k0` or `k1` for [`SiphashRng::new_with_keys`] (with the other parameter being 0).
    pub const fn from_seed(seed: u64) -> Self {
        Self::new_with_keys(
            seed ^ 0x6a09e667f3bcc908,
            seed.rotate_right(31) ^ 0xbb67ae8584caa73b,
        )
    }

    /// Convience function for constructing a [`SipHashRng`] from an arbitrary seed. The resulting generator has a maximum of `st.len() * 8` bits of enthropy (up to 256 bits)
    pub fn from_word_seed(st: &[u8]) -> Self {
        let mut base = RawSipHasher::from_keys(0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1);

        base.update(st.len() as u64);
        base.update_from_bytes(st);

        Self(base)
    }

    /// Low-level function for producing a new psuedo-random value and updating the generator by writing `word0` before computing the result, then word1 after.
    pub fn tick_with_ingest(&mut self, word0: u64, word1: u64) -> u64 {
        self.0.update(word0);
        let val = self.0.finish();
        self.0.update(word1);
        val
    }

    /// Ticks the generator.and produces a pseudorandom value.
    pub fn tick(&mut self) -> u64 {
        self.tick_with_ingest(0x510e527fade682d1, 0x9b05688c2b3e6c1f)
    }

    /// Returns a reference to the raw inner value
    pub fn raw(&self) -> &RawSipHasher<C, D> {
        &self.0
    }

    /// Returns a mutable reference to the raw inner state. Modifying this state affects the sequence of random numbers produced by the [`SiphashRng::tick`] function.
    pub fn raw_mut(&mut self) -> &mut RawSipHasher<C, D> {
        &mut self.0
    }
}

#[cfg(feature = "rand_core")]
mod imp {
    use rand_core::*;

    use crate::rng::SiphashRng;

    impl<const C: usize, const D: usize> RngCore for SiphashRng<C, D> {
        fn next_u64(&mut self) -> u64 {
            self.tick()
        }

        fn next_u32(&mut self) -> u32 {
            let val = self.tick();

            val as u32 ^ (val >> 32) as u32
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            rand_core::impls::fill_bytes_via_next(self, dest);
        }
    }

    impl<const C: usize, const D: usize> SeedableRng for SiphashRng<C, D> {
        type Seed = [u8; 16];
        fn from_seed(seed: Self::Seed) -> Self {
            let [k0, k1] = unsafe { core::mem::transmute(seed) };

            Self::new_with_keys(k0, k1)
        }

        fn from_rng(rng: &mut impl RngCore) -> Self {
            Self::new_with_keys(rng.next_u64(), rng.next_u64())
        }

        fn seed_from_u64(state: u64) -> Self {
            Self::from_seed(state)
        }
    }
}
