#[cfg(target_feature = "sse3")]
#[allow(improper_ctypes_definitions)] // using default ABI was causing extraneous moves into memory, even when functions got inlined
mod sse {

    use super::super::*;

    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;

    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    #[cfg(target_feature = "avx2")]
    #[inline]
    fn rotate_lanes_epi64(v: __m128i, count: __m128i) -> __m128i {
        let lshift = count;
        let rshift = unsafe { _mm_sub_epi64(_mm_set_epi64x(64, 64), count) };

        let left = unsafe { _mm_sllv_epi64(v, lshift) };
        let right = unsafe { _mm_srlv_epi64(v, rshift) };

        unsafe { _mm_or_si128(left, right) }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    fn rotate_lanes_epi64(v: __m128i, count: __m128i) -> __m128i {
        let lshift = count;
        let rshift = unsafe { _mm_sub_epi64(_mm_set_epi64x(64, 64), count) };

        let lsh1 = unsafe { _mm_unpacklo_epi64(lshift, lshift) };
        let lsh2 = unsafe { _mm_sub_epi64(_mm_unpackhi_epi64(lshift, lshift), lsh1) };
        let rsh1 = unsafe { _mm_unpackhi_epi64(rshift, rshift) };
        let rsh2 = unsafe { _mm_sub_epi64(_mm_unpacklo_epi64(rshift, rshift), rsh1) };

        let int1 = unsafe { _mm_sll_epi64(v, lsh1) };
        let int2 = unsafe { _mm_sll_epi64(int1, lsh2) };
        let int3 = unsafe { _mm_srl_epi64(v, rsh1) };
        let int4 = unsafe { _mm_srl_epi64(int3, rsh2) };

        let int5 = unsafe { _mm_shuffle_epi32(int2, 0x1E) };
        let int6 = unsafe { _mm_shuffle_epi32(int4, 0x1E) };

        let sll_res = unsafe { _mm_unpacklo_epi64(int1, int5) };
        let srl_res = unsafe { _mm_unpacklo_epi64(int3, int6) };

        unsafe { _mm_or_si128(sll_res, srl_res) }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct SipHashState(__m128i, __m128i);

    impl SipHashState {
        #[inline]
        pub const fn from_keys(k0: u64, k1: u64) -> Self {
            let s0 = unsafe { core::mem::transmute([k0 ^ SIPHASH_MAG1, k0 ^ SIPHASH_MAG3]) };
            let s1 = unsafe { core::mem::transmute([k1 ^ SIPHASH_MAG2, k1 ^ SIPHASH_MAG4]) };

            Self(s0, s1)
        }

        #[inline]
        pub fn update_before_rounds(&mut self, word: u64) {
            let val: __m128i = unsafe { _mm_set_epi64x(word as i64, 0) };
            self.1 = unsafe { _mm_xor_si128(self.1, val) };
        }

        #[inline]
        pub fn update_after_rounds(&mut self, word: u64) {
            let val: __m128i = unsafe { _mm_set_epi64x(0, word as i64) };
            self.0 = unsafe { _mm_xor_si128(self.0, val) };
        }

        #[inline]
        pub fn update_before_final(&mut self) {
            let val: __m128i = unsafe { _mm_set_epi64x(0xff, 0) };
            self.0 = unsafe { _mm_xor_si128(self.0, val) };
        }

        #[inline]
        pub fn finish(mut self) -> u64 {
            self.0 = unsafe { _mm_xor_si128(self.0, self.1) };
            let [l, h]: [u64; 2] = unsafe { core::mem::transmute(self.0) };
            l ^ h
        }

        #[inline]
        extern "sysv64" fn halfround(
            mut s0: __m128i,
            mut s1: __m128i,
            rotate: __m128i,
        ) -> (__m128i, __m128i) {
            // Compute one half of the round function.with [v0,v2] in s0, and [v1,v3] in s1, and [rot1,rot3] in rotate
            // The halfround function is (in scalar ops):
            //  v0 = v0 + v1
            //  v2 = v2 + v3
            //  v1 = v1 rrot rot1
            //  v3 = v3 rrot rot3
            //  v1 = v1 ^ v0
            //  v3 = v3 ^ v2
            //  v0 = v0 rrot 32
            //  (v2,v0) = (v0,v2)
            // We vectorize by combining each pair of steps into u64x2 SIMD ops via x86_64 SIMD intrinsics
            // A full round is 2 halfrounds, the first with [rot1,rot3] = [13, 16], and the second with [rot1, rot3] = [17, 21]
            s0 = unsafe { _mm_add_epi64(s0, s1) };
            s1 = rotate_lanes_epi64(s1, rotate);
            s1 = unsafe { _mm_xor_si128(s1, s0) };
            // permute [v0l,v0h,v2l,v2h] as u32x4 instead of u64x2 to [v2l,v2h, v0h, v0l] - this rotates v0 32 bits, and then swaps them setting up for the second halfround
            // or resetting for next full round
            s0 = unsafe { _mm_shuffle_epi32(s0, 0b0_01_11_10) };

            (s0, s1)
        }

        #[inline]
        pub fn round(&mut self) {
            // s0 = [v0,v2], s1 = [v1,v3]
            let Self(s0, s1) = *self;
            // `_mm_set_epi64x` has reversed parameter order - yields [rot1, rot3] = [13, 16]
            let (s0, s1) = Self::halfround(s0, s1, unsafe { _mm_set_epi64x(16, 13) });
            // [rot1,rot3] = [17,21]
            let (s0, s1) = Self::halfround(s0, s1, unsafe { _mm_set_epi64x(21, 17) });
            *self = Self(s0, s1);
        }
    }
}

#[cfg(target_feature = "sse3")]
pub use sse::*;

#[cfg(not(target_feature = "sse3"))]
include!("generic.rs");
