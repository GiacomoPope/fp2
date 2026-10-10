//! Universal guarded AArch64 NEON Montgomery multiplication kernel.
//!
//! Columns use a unified guarded schedule. Smaller kernels retain fixed-bound
//! product loops; kernels with at least 24 limbs explicitly expand their terms.

use core::arch::aarch64::*;

/// Multiplies or squares four radix-`2^W` sparse-Montgomery values, returning
/// normalized integers below `2p`, where `p = d*2^(W*(K-1)) - 1`.
///
/// # Safety
///
/// NEON must be available. Inputs must be normalized and below `2p`, with the
/// sparse modulus and Montgomery headroom validated by the caller. In square
/// mode both inputs must represent the same values.
#[target_feature(enable = "neon")]
#[inline(never)]
#[allow(unused_assignments)] // The final syntactic quotient write is discarded for supported K.
pub unsafe fn multiply<const K: usize, const W: usize, const SQUARE: bool>(
    a: &[uint32x4_t; K],
    b: &[uint32x4_t; K],
    d: u32,
) -> [uint32x4_t; K] {
    const {
        assert!(matches!(K, 11 | 14 | 17 | 20 | 24 | 27));
        assert!(matches!(W, 27 | 28 | 29 | 30));
        assert!(W != 30 || K == 11);
    }

    let zero = vdupq_n_u64(0);
    let mask = vdupq_n_u64((1u64 << W) - 1);
    let mut carry_lo = zero;
    let mut carry_hi = zero;
    let mut quotient = [[vdup_n_u32(0); 2]; K];
    let mut out = [vdupq_n_u32(0); K];

    macro_rules! terms {
        ($term:ident; $($i:literal),*) => { $($term!($i);)* };
    }
    macro_rules! column {
        ($col:literal) => {
            if $col < 2 * K - 1 {
                let mut lo = carry_lo;
                let mut hi = carry_hi;

                macro_rules! term {
                    ($i:expr) => {
                        if $i < K && $i <= $col && $col - $i < K && (!SQUARE || $i <= $col - $i)
                        {
                            let av = a[$i];
                            let bv = b[$col - $i];
                            let product_lo = vmull_u32(vget_low_u32(av), vget_low_u32(bv));
                            let product_hi = vmull_high_u32(av, bv);
                            lo = vaddq_u64(lo, product_lo);
                            hi = vaddq_u64(hi, product_hi);
                            if SQUARE && $i < $col - $i {
                                lo = vaddq_u64(lo, product_lo);
                                hi = vaddq_u64(hi, product_hi);
                            }
                        }
                    };
                }

                if K < 24 {
                    // Enumerate only this diagonal's valid indices, with the
                    // same fixed bounds as the original NEON product loops.
                    let start = ($col as usize).saturating_sub(K - 1);
                    let end = core::cmp::min($col + 1, K);
                    for i in start..end {
                        term!(i);
                    }
                } else {
                    // Larger NEON kernels benefit from explicit expansion.
                    terms!(term;
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                        14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26
                    );
                }

                if $col >= K - 1 {
                    lo = vmlal_n_u32(lo, quotient[$col - (K - 1)][0], d);
                    hi = vmlal_n_u32(hi, quotient[$col - (K - 1)][1], d);
                }

                let low = vmovn_u64(vandq_u64(lo, mask));
                let high = vmovn_u64(vandq_u64(hi, mask));
                if $col < K {
                    quotient[$col] = [low, high];
                } else {
                    out[$col - K] = vcombine_u32(low, high);
                }
                carry_lo = unsafe { shr_w::<W>(lo) };
                carry_hi = unsafe { shr_w::<W>(hi) };
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

    let low = vmovn_u64(vandq_u64(carry_lo, mask));
    let high = vmovn_u64(vandq_u64(carry_hi, mask));
    out[K - 1] = vcombine_u32(low, high);
    out
}

#[inline(always)]
unsafe fn shr_w<const W: usize>(x: uint64x2_t) -> uint64x2_t {
    unsafe {
        if W == 27 {
            vshrq_n_u64::<27>(x)
        } else if W == 28 {
            vshrq_n_u64::<28>(x)
        } else if W == 29 {
            vshrq_n_u64::<29>(x)
        } else {
            vshrq_n_u64::<30>(x)
        }
    }
}
