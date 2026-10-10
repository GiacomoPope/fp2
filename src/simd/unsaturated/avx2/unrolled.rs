//! Universal guarded AVX2 sparse Montgomery multiplication and squaring kernel.
//!
//! A single column schedule covers all supported limb counts. Compile-time
//! guards discard terms outside the selected layout.

use core::arch::x86_64::*;

/// Multiplies or squares four field values in parallel.
///
/// # Safety
///
/// The caller must ensure that AVX2 is available. The input arrays must contain
/// normalized radix-2^W integers below 2p, and the sparse modulus and available
/// headroom must have been validated by UnsaturatedContext::new.
#[target_feature(enable = "avx2")]
#[inline(never)]
#[allow(unused_assignments)] // The final syntactic quotient write is discarded for supported K.
pub(super) unsafe fn multiply<const K: usize, const W: usize, const SQUARE: bool>(
    a: &[__m128i; K],
    b: &[__m128i; K],
    d: u32,
) -> [__m128i; K] {
    const {
        assert!(matches!(K, 11 | 14 | 17 | 20 | 24 | 27));
        assert!(matches!(W, 27 | 28 | 29 | 30));
        assert!(W != 30 || K == 11);
    }

    let zero = _mm256_setzero_si256();
    let mut aa = [zero; K];
    let mut bb = [zero; K];
    // Do not use array::map here: its closures can lose the AVX2 feature
    // boundary and become out-of-line calls for each limb.
    for i in 0..K {
        aa[i] = _mm256_cvtepu32_epi64(a[i]);
        bb[i] = if SQUARE {
            aa[i]
        } else {
            _mm256_cvtepu32_epi64(b[i])
        };
    }

    let mask = _mm256_set1_epi64x((1i64 << W) - 1);
    let shift = _mm_cvtsi32_si128(W as i32);
    let factor = _mm256_set1_epi64x(d as i64);
    let mut quotient = [zero; K];
    let mut out = [_mm_setzero_si128(); K];
    let mut carry = zero;

    macro_rules! terms {
        ($term:ident; $($i:literal),*) => { $($term!($i);)* };
    }
    macro_rules! column {
        ($col:literal) => {
            if $col < 2 * K - 1 {
                let mut sum = carry;

                macro_rules! term {
                    ($i:literal) => {
                        if $i < K && $i <= $col && $col - $i < K && (!SQUARE || $i <= $col - $i) {
                            let j = $col - $i;
                            let product = _mm256_mul_epu32(aa[$i], bb[j]);
                            sum = _mm256_add_epi64(sum, product);
                            if SQUARE && $i < j {
                                sum = _mm256_add_epi64(sum, product);
                            }
                        }
                    };
                }

                terms!(term;
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                    14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26
                );

                if $col >= K - 1 {
                    sum = _mm256_add_epi64(sum, _mm256_mul_epu32(quotient[$col - (K - 1)], factor));
                }

                let digit = _mm256_and_si256(sum, mask);
                if $col < K {
                    quotient[$col] = digit;
                } else {
                    out[$col - K] = unsafe { narrow(digit) };
                }
                carry = _mm256_srl_epi64(sum, shift);
            }
        };
    }
    macro_rules! columns {
        ($($col:literal),*) => { $(column!($col);)* };
    }
    columns!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52
    );

    out[K - 1] = unsafe { narrow(carry) };
    out
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn narrow(v: __m256i) -> __m128i {
    let v = _mm256_shuffle_epi32::<0x88>(v);
    _mm_unpacklo_epi64(_mm256_castsi256_si128(v), _mm256_extracti128_si256::<1>(v))
}
