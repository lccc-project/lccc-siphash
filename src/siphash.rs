use core::hash::Hasher;

const SIPHASH_MAG1: u64 = 0x736f6d6570736575;
const SIPHASH_MAG2: u64 = 0x646f72616e646f6d;
const SIPHASH_MAG3: u64 = 0x6c7967656e657261;
const SIPHASH_MAG4: u64 = 0x7465646279746573;

#[cfg_attr(target_arch = "clever", path = "siphash/clever.rs")]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    path = "siphash/x86.rs"
)]
#[cfg_attr(
    not(any(target_arch = "clever", target_arch = "x86", target_arch = "x86_64")),
    path = "siphash/generic.rs"
)]
pub mod sys;

#[derive(Copy, Clone, Debug)]
pub struct SipHasher<const C: usize, const D: usize> {
    state: sys::SipHashState,
    tail: u64,
    ntail: usize,
    bytes: usize,
}

impl<const C: usize, const D: usize> SipHasher<C, D> {
    pub const fn new_with_keys(k0: u64, k1: u64) -> Self {
        Self {
            state: sys::SipHashState::from_keys(k0, k1),
            tail: 0u64,
            ntail: 0,
            bytes: 0,
        }
    }

    pub fn update(&mut self, word: u64) {
        self.state.update_before_rounds(word);
        for _ in 0..C {
            self.state.round();
        }
        self.state.update_after_rounds(word);
    }
}

impl<const C: usize, const D: usize> Hasher for SipHasher<C, D> {
    fn write(&mut self, mut s: &[u8]) {
        self.bytes += s.len();
        if self.ntail > 0 {
            let required = s.len().min(8 - self.ntail);
            let (l, r) = s.split_at(required);

            (unsafe { core::mem::transmute::<&mut u64, &mut [u8; 8]>(&mut self.tail) })
                [self.ntail..][..required]
                .copy_from_slice(l);

            s = r;
            if required + self.ntail == 8 {
                self.update(self.tail.to_le());
                self.ntail = 0;
            }
        }

        let chunks_exact = s.chunks_exact(8);
        let remainder = chunks_exact.remainder();
        for c in chunks_exact {
            let word = u64::from_le_bytes(unsafe { *(c as *const [u8] as *const [u8; 8]) });

            self.update(word);
        }

        if !remainder.is_empty() {
            let mut bytes = [0u8; 8];
            self.ntail = remainder.len();
            bytes[..self.ntail].copy_from_slice(remainder);
            self.tail = u64::from_le_bytes(bytes);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        let mut state = *self;
        if self.ntail > 0 {
            let mut word = self.tail.to_le();

            if cfg!(target_endian = "big") {
                word >>= (8 - self.ntail) << 3;
            }

            word &= (2u64 << ((self.ntail) << 3) - 1) - 1;

            if self.ntail != 8 {
                word |= ((self.bytes as u64) & 0xFF) << 56;
            }

            state.update(word);
        } else {
            state.update(((self.bytes as u64) & 0xFF) << 56);
        }
        state.state.update_before_final();

        for _ in 0..D {
            state.state.round();
        }

        state.state.finish().to_le()
    }
}
