//! # NEON representation and lane operations
//!
//! Each limb is stored in a `u32x4` NEON vector, with one lane for each of the
//! four transposed field values. Multiplication widens the low and high pairs
//! into separate 64-bit accumulators using NEON's native 32-by-32-bit widening
//! instructions; NEON has no equivalent widening 64-bit integer multiply.
//!
//! Butterflies use NEON lane swaps, packed sums and differences, and vector
//! selection for the pairwise transforms used by Hadamard operations. This
//! avoids constructing packs of duplicated operands. Lane indices and reduction
//! schedules are public; choices based on field values use masks and vector
//! selects rather than branches. Functional tests do not by themselves prove
//! constant-time behavior.

use core::arch::aarch64::*;
use core::marker::PhantomData;

use fearless_simd::{aarch64::Neon, prelude::*, u32x4};

use super::{LimbOps, MontgomeryField, UnsaturatedContext, UnsaturatedPacked};
use crate::packed::PackedField;

mod unrolled;
use unrolled::multiply;

struct NeonOps;

impl LimbOps<Neon> for NeonOps {
    type Raw = uint32x4_t;

    #[inline(always)]
    fn zero() -> Self::Raw {
        unsafe { vdupq_n_u32(0) }
    }

    #[inline(always)]
    fn splat(value: u32) -> Self::Raw {
        unsafe { vdupq_n_u32(value) }
    }

    #[inline(always)]
    fn add(a: Self::Raw, b: Self::Raw) -> Self::Raw {
        unsafe { vaddq_u32(a, b) }
    }

    #[inline(always)]
    fn sub(a: Self::Raw, b: Self::Raw) -> Self::Raw {
        unsafe { vsubq_u32(a, b) }
    }

    #[inline(always)]
    fn and(a: Self::Raw, b: Self::Raw) -> Self::Raw {
        unsafe { vandq_u32(a, b) }
    }

    #[inline(always)]
    fn shift_right<const W: usize>(value: Self::Raw) -> Self::Raw {
        unsafe { vshlq_u32(value, vdupq_n_s32(-(W as i32))) }
    }

    #[inline(always)]
    fn borrow(value: Self::Raw) -> Self::Raw {
        unsafe { vshrq_n_u32::<31>(value) }
    }

    #[inline(always)]
    fn reduction_select(borrow: Self::Raw) -> Self::Raw {
        unsafe { vceqq_u32(borrow, vdupq_n_u32(0)) }
    }

    #[inline(always)]
    fn select(select: Self::Raw, reduced: Self::Raw, original: Self::Raw) -> Self::Raw {
        unsafe { vbslq_u32(select, reduced, original) }
    }

    #[inline(always)]
    fn from_u32x4(value: u32x4<Neon>) -> Self::Raw {
        value.into()
    }

    #[inline(always)]
    fn to_u32x4(simd: Neon, value: Self::Raw) -> u32x4<Neon> {
        u32x4::simd_from(simd, value)
    }
}

type Context<F, const L: usize, const LOW: usize, const K: usize, const W: usize> =
    UnsaturatedContext<F, Neon, L, LOW, K, W>;

impl<F: MontgomeryField, const L: usize, const LOW: usize, const K: usize, const W: usize>
    Context<F, L, LOW, K, W>
{
    #[inline(always)]
    fn product<const SQUARE: bool>(
        &self,
        a: &UnsaturatedPacked<F, Neon, K>,
        b: &UnsaturatedPacked<F, Neon, K>,
    ) -> UnsaturatedPacked<F, Neon, K> {
        unsafe {
            let aa: [uint32x4_t; K] = a.limbs.map(Into::into);
            let bb;
            let b = if SQUARE {
                &aa
            } else {
                bb = b.limbs.map(Into::into);
                &bb
            };
            let raw = multiply::<K, W, SQUARE>(&aa, b, self.modulus_digits[K - 1] + 1);
            UnsaturatedPacked {
                limbs: raw.map(|x| u32x4::simd_from(self.simd, x)),
                field: PhantomData,
            }
        }
    }
}

impl<F: MontgomeryField, const L: usize, const LOW: usize, const K: usize, const W: usize>
    PackedField for Context<F, L, LOW, K, W>
{
    type Scalar = F;
    type Packed = UnsaturatedPacked<F, Neon, K>;

    fn is_accelerated(&self) -> bool {
        true
    }

    fn vectorize<R>(&self, f: impl FnOnce() -> R) -> R {
        self.simd.vectorize(f)
    }

    fn pack(&self, values: [F; 4]) -> Self::Packed {
        self.simd.vectorize(
            #[inline(always)]
            || self.pack_values(values),
        )
    }

    fn unpack(&self, value: &Self::Packed) -> [F; 4] {
        self.simd.vectorize(
            #[inline(always)]
            || {
                let canonical = self.reduce_with::<NeonOps>(value, &self.modulus_digits);
                self.unpack_canonical(&canonical)
            },
        )
    }

    fn add(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        self.add_mod_with::<NeonOps>(a, b)
    }

    fn sub(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        self.sub_mod_with::<NeonOps>(a, b)
    }

    fn mul(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        self.product::<false>(a, b)
    }

    fn square(&self, a: &Self::Packed) -> Self::Packed {
        self.product::<true>(a, a)
    }

    fn gather<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        &self,
        values: &[Self::Packed],
    ) -> Self::Packed {
        self.gather_values::<I0, I1, I2, I3>(values)
    }

    fn butterfly<const BIT: usize>(&self, value: &Self::Packed) -> Self::Packed {
        assert!(BIT == 1 || BIT == 2);
        unsafe {
            let mut swapped = self.zero();
            for i in 0..K {
                let v: uint32x4_t = value.limbs[i].into();
                let s = if BIT == 1 {
                    vrev64q_u32(v)
                } else {
                    vextq_u32::<2>(v, v)
                };
                swapped.limbs[i] = u32x4::simd_from(self.simd, s);
            }
            let sum = self.add_mod_with::<NeonOps>(value, &swapped);
            // In the upper lane of each pair, swapped - original is the
            // required lower-minus-upper difference.
            let difference = self.sub_mod_with::<NeonOps>(&swapped, value);
            let select: uint32x4_t = u32x4::simd_from(
                self.simd,
                core::array::from_fn::<_, 4, _>(|i| if i & BIT == 0 { u32::MAX } else { 0 }),
            )
            .into();
            UnsaturatedPacked {
                limbs: core::array::from_fn(|i| {
                    u32x4::simd_from(
                        self.simd,
                        vbslq_u32(select, sum.limbs[i].into(), difference.limbs[i].into()),
                    )
                }),
                field: PhantomData,
            }
        }
    }
}
