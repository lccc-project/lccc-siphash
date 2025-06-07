use crate::RawSipHasher;

#[derive(Copy, Clone, Debug)]
pub struct SiphashRng<const C: usize, const D: usize>(RawSipHasher<C, D>);

impl<const C: usize, const D: usize> SiphashRng<C, D> {
    pub const fn new_with_keys(k0: u64, k1: u64) -> Self {
        Self(RawSipHasher::from_keys(k0, k1))
    }

    pub const fn from_raw(raw: RawSipHasher<C, D>) -> Self {
        Self(raw)
    }

    pub const fn from_seed(seed: u64) -> Self {
        Self::new_with_keys(
            seed ^ 0x6a09e667f3bcc908,
            seed.rotate_right(31) ^ 0xbb67ae8584caa73b,
        )
    }

    pub fn from_word_seed(st: &[u8]) -> Self {
        let mut base = RawSipHasher::from_keys(0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1);

        base.update(st.len() as u64);
        base.update_from_bytes(st);

        Self(base)
    }

    pub fn tick_with_ingest(&mut self, word0: u64, word1: u64) -> u64 {
        self.0.update(word0);
        let val = { *self }.0.finish();
        self.0.update(word1);
        val
    }

    pub fn tick(&mut self) -> u64 {
        self.tick_with_ingest(0x510e527fade682d1, 0x9b05688c2b3e6c1f)
    }

    pub fn raw(&self) -> &RawSipHasher<C, D> {
        &self.0
    }

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

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            let mut exact = dst.chunks_exact_mut(8);

            for chunk in &mut exact {
                let v = self.tick();

                chunk.copy_from_slice(&v.to_ne_bytes());
            }

            let remainder = exact.into_remainder();

            if !remainder.is_empty() {
                let last = self.tick();
                let bytes = last.to_le_bytes();
                let len = remainder.len();
                remainder.copy_from_slice(&bytes[..len]);
            }
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
