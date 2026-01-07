//! Module providing primary implementations of SipHash primitives and hashers
//!
//! This module implements the algorithm defined by <https://eprint.iacr.org/2012/351.pdf>, using generic parameters for *C* and *D* defined therein.
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
mod sys;

/// Raw state implementation of SipHash.
/// This wraps a target-dependant state type to provide primitive operations like the SipHash round function.
///
/// Logically, the [`SipHashState`] is the state array `[s0, s1, s2, s3]`.
/// However no guarantee is made about the precise layout (notably, on many targets, the implementation stores this as `[s0, s2, s1, s3]` to make SIMD operations work nicer).
/// Use [`SipHashState::inspect_state`] and [`SipHashState::from_state`] as primitives to access the underlying state array.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct SipHashState(sys::SipHashState);

impl SipHashState {
    /// Constructs a new [`SipHashState`] from the specified keys.
    /// Per the specification, this initializes the state to `[k0 ^ 0x736f6d6570736575, k1 ^ 0x646f72616e646f6d, k0 ^ 0x6c7967656e657261, k1 ^ 0x7465646279746573]`
    #[inline]
    pub const fn from_keys(k0: u64, k1: u64) -> Self {
        Self(sys::SipHashState::from_keys(k0, k1))
    }

    /// Constructs a new [`SipHashState`] from the state array.
    #[inline]
    pub const fn from_state(state: [u64; 4]) -> Self {
        Self(sys::SipHashState::from_state(state))
    }

    /// Returns the current state array of the [`SipHashState`]. This function is intended for serialization and for debugging only (not to modify the state array)
    #[inline]
    pub const fn inspect_state(&self) -> [u64; 4] {
        self.0.inspect_state()
    }

    /// Performs the update operation to injest `word` before applying the update rounds to the state array.
    /// This mutates the state by xoring `word` into s3.
    #[inline]
    pub fn update_before_rounds(&mut self, word: u64) {
        self.0.update_before_rounds(word);
    }

    /// Performs the update operation to injest `word` after applying the update rounds to the state array.
    /// This mutates the state by xoring `word` into s0.
    #[inline]
    pub fn update_after_rounds(&mut self, word: u64) {
        self.0.update_after_rounds(word);
    }

    /// Performs the update operation before the finalization rounds.
    /// This mutates the state by xoring `0xff` into s2.
    #[inline]
    pub fn update_before_final(&mut self) {
        self.0.update_before_final();
    }

    /// Consumes the state and produces the final value.
    ///
    /// This xors each word of the state array together and returns them. Note that this does not apply the finalization rounds, and you must perform these rounds manually (or call [`update_and_final`])
    #[inline]
    pub fn finish(self) -> u64 {
        self.0.finish()
    }

    /// Performs a single SipHash round operation on the state array.
    #[inline]
    pub fn round(&mut self) {
        self.0.round();
    }

    /// Injests `val` and then performs `R` siphash rounds.
    /// Convience wrapper for calling [`Self::update_before_rounds`], then R calls to [`Self::round`], then a call to [`Self::update_after_rounds`]
    pub fn update_and_round<const R: usize>(&mut self, val: u64) {
        self.update_before_rounds(val);
        for _ in 0..R {
            self.round();
        }
        self.update_after_rounds(val);
    }

    /// Consumes the state, and performs the full finalization step with R finalization rounds.
    /// Convience wrapper arround consuming the value (copying if necessary),
    ///  calling [`Self::update_before_final`], then R calls to [`Self::round`], then a call to [`Self::finish`]
    pub fn update_and_final<const R: usize>(mut self) -> u64 {
        self.update_before_final();
        for _ in 0..R {
            self.round();
        }
        self.finish()
    }
}

#[cfg(feature = "serde")]
mod serde;

/// A raw [`Hasher`] that directly wraps a [`SipHashState`]. There are two sets of operations provided:
/// * Direct Operations, and
/// * [`Hasher`] operations.
///
/// Both have similar behaviour, and operate on chunks of 8 bytes. Unlike [`SipHasher`],
/// [`RawSipHasher`] does not buffer bytes internally, and instead pads out values and byte arrays to 8 bytes.
/// This is more efficient than [`SipHasher`], especially when all written values are 8 bytes long,
/// but produces different results from [`SipHasher`] when several smaller values are hashed, and may produce values that differ from other, similar operations, in surprising ways.
/// As a notable example, hashing `[u32; 4]` would produce a different result than hashing the bytes of that same array (even on little-endian platforms).
///
/// `C` and `D` are the parameters of SipHash-*C*-*D*. It is recommended that these values be small, but they can be arbitrary.
///
/// Generally, C=2, and D=4 provides sufficient security for any use case, and C=1 and D=3 can produce a more efficient algorithm with lower security (though still sufficient for many use cases).
#[derive(Copy, Clone, Debug)]
pub struct RawSipHasher<const C: usize, const D: usize>(SipHashState);

impl<const C: usize, const D: usize> RawSipHasher<C, D> {
    /// Constructs a new [`RawSipHasher`]. This constructs the internal state as if by [`SipHashState::from_keys`]
    pub const fn from_keys(k0: u64, k1: u64) -> Self {
        Self(SipHashState::from_keys(k0, k1))
    }

    #[cfg(not(feature = "inspect-raw"))]
    const fn from_state(state: SipHashState) -> Self {
        Self(state)
    }

    /// Constructs a [`RawSipHasher`] that wraps a given internal state.
    #[cfg(feature = "inspect-raw")]
    #[cfg_attr(feature = "nightly-docs", doc(cfg(feature = "inspect-raw")))]
    pub const fn from_state(state: SipHashState) -> Self {
        Self(state)
    }

    #[cfg(not(feature = "inspect-raw"))]
    const fn state(&self) -> &SipHashState {
        &self.0
    }

    /// Obtains the inner state for the purposes of debugging and serialization.
    #[cfg(feature = "inspect-raw")]
    #[cfg_attr(feature = "nightly-docs", doc(cfg(feature = "inspect-raw")))]
    pub const fn state(&self) -> &SipHashState {
        &self.0
    }

    /// Updates the state by writing a word, and performing `C` rounds.
    pub fn update(&mut self, word: u64) {
        self.0.update_and_round::<C>(word)
    }

    /// Finishes the Hash by performing the finalization steps of a fresh copy of the state, before producing the final value of the hash
    pub fn finish(&self) -> u64 {
        let state = *self;
        state.0.update_and_final::<D>().to_le()
    }

    /// Updates the hash using each 8 byte chunk of `bytes`, padding the remainder (if any) with 0 bytes.
    pub fn update_from_bytes(&mut self, bytes: &[u8]) {
        let (chunks, rem) = bytes.as_chunks::<8>();
        for &chunk in chunks {
            self.update(u64::from_le_bytes(chunk));
        }

        let mut v = [0x00; 8];
        v[..rem.len()].copy_from_slice(rem);
        if !rem.is_empty() {
            self.update(u64::from_le_bytes(v))
        }
    }

    /// Updates the hash using each 8 byte chunk of `st`, padding the remainder with a minimum of 1 0xFF byte.
    ///
    /// When the `nightly-prefixfree_extras` feature is enabled, [`Hasher::write_str`] has the same effect as this function
    pub fn update_from_string(&mut self, st: &str) {
        let bytes = st.as_bytes();
        let (chunks, rem) = bytes.as_chunks::<8>();
        for &chunk in chunks {
            self.update(u64::from_le_bytes(chunk));
        }

        let mut v = [0xFF; 8];
        v[..rem.len()].copy_from_slice(rem);

        self.update(u64::from_le_bytes(v))
    }
}

impl<const C: usize, const D: usize> Hasher for RawSipHasher<C, D> {
    fn finish(&self) -> u64 {
        self.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update_from_bytes(bytes);
    }

    fn write_u64(&mut self, i: u64) {
        self.update(i);
    }

    fn write_u128(&mut self, i: u128) {
        self.update(i as u64);
        self.update((i >> 64) as u64);
    }

    fn write_i128(&mut self, i: i128) {
        self.write_u128(i as u128);
    }

    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64)
    }

    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64)
    }

    fn write_isize(&mut self, i: isize) {
        self.write_u64(i as u64)
    }

    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64)
    }

    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64)
    }

    fn write_i32(&mut self, i: i32) {
        self.write_u64(i as u64)
    }

    fn write_i16(&mut self, i: i16) {
        self.write_u64(i as u64)
    }

    fn write_i8(&mut self, i: i8) {
        self.write_u64(i as u64)
    }

    #[cfg(feature = "nightly-prefixfree_extras")]
    fn write_str(&mut self, s: &str) {
        self.update_from_string(s);
    }
}

/// [`SipHasher`] is a complete implementation of SipHash, including
#[derive(Copy, Clone, Debug)]
pub struct SipHasher<const C: usize, const D: usize> {
    state: SipHashState,
    tail: u64,
    ntail: usize,
    bytes: usize,
}

impl<const C: usize, const D: usize> SipHasher<C, D> {
    /// Constructs a new [`SipHasher`] from a default state using keys k0 and k1
    pub const fn new_with_keys(k0: u64, k1: u64) -> Self {
        Self {
            state: SipHashState::from_keys(k0, k1),
            tail: 0u64,
            ntail: 0,
            bytes: 0,
        }
    }

    /// Convience function that updates the state with the specified word
    pub fn update(&mut self, word: u64) {
        self.state.update_and_round::<C>(word)
    }

    /// Obtains the underlying raw [`SipHashState`]
    #[cfg(feature = "inspect-raw")]
    #[cfg_attr(feature = "nightly-docs", doc(cfg(feature = "inspect-raw")))]
    pub const fn state(&self) -> &SipHashState {
        &self.state
    }

    /// Constructs a new [`SipHasher`] from the specified raw state. Note that the current hash state outside of the [`SipHashState`] is not preserved,
    ///   and any words that have not yet been finished are discarded in a roundtrip through this function and [`SipHasher::state`].
    #[cfg(feature = "inspect-raw")]
    #[cfg_attr(feature = "nightly-docs", doc(cfg(feature = "inspect-raw")))]
    pub const fn from_state(state: SipHashState) -> Self {
        Self {
            state,
            tail: 0,
            ntail: 0,
            bytes: 0,
        }
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
            } else {
                return;
            }
        }

        let (chunks, remainder) = s.as_chunks::<8>();

        for &chunk in chunks {
            self.update(u64::from_ne_bytes(chunk));
        }

        let mut tail = [0u8; 8];
        tail[..remainder.len()].copy_from_slice(remainder);
        self.ntail = remainder.len();
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
        state.state.update_and_final::<D>().to_le()
    }

    #[cfg(feature = "nightly-prefixfree_extras")]
    fn write_str(&mut self, s: &str) {
        self.write(s.as_bytes());
        let word = self.tail.to_le() | (!0) << (self.ntail << 3);
        self.update(word);
        self.tail = 0;
        self.ntail = 0;
    }
}
