//! AVX2 sparse Montgomery arithmetic. Each widening multiply processes all four
//! field values in the low halves of four 64-bit lanes. Storage remains four
//! 32-bit lanes per limb, matching the shared unsaturated representation.

use core::arch::x86_64::*;
use core::marker::PhantomData;

use fearless_simd::{prelude::*, u32x4, x86::Avx2};

use super::{LimbOps, MontgomeryField, UnsaturatedContext, UnsaturatedPacked};
use crate::packed::PackedField;

mod unrolled;
use unrolled::multiply;

struct Avx2Ops;

impl LimbOps<Avx2> for Avx2Ops {
    type Raw = __m128i;

    #[inline(always)]
    fn zero() -> Self::Raw {
        unsafe { _mm_setzero_si128() }
    }

    #[inline(always)]
    fn splat(value: u32) -> Self::Raw {
        unsafe { _mm_set1_epi32(value as i32) }
    }

    #[inline(always)]
    fn add(a: Self::Raw, b: Self::Raw) -> Self::Raw {
        unsafe { _mm_add_epi32(a, b) }
    }

    #[inline(always)]
    fn sub(a: Self::Raw, b: Self::Raw) -> Self::Raw {
        unsafe { _mm_sub_epi32(a, b) }
    }

    #[inline(always)]
    fn and(a: Self::Raw, b: Self::Raw) -> Self::Raw {
        unsafe { _mm_and_si128(a, b) }
    }

    #[inline(always)]
    fn shift_right<const W: usize>(value: Self::Raw) -> Self::Raw {
        unsafe { _mm_srl_epi32(value, _mm_cvtsi32_si128(W as i32)) }
    }

    #[inline(always)]
    fn borrow(value: Self::Raw) -> Self::Raw {
        unsafe { _mm_srli_epi32::<31>(value) }
    }

    #[inline(always)]
    fn reduction_select(borrow: Self::Raw) -> Self::Raw {
        unsafe { _mm_sub_epi32(_mm_setzero_si128(), borrow) }
    }

    #[inline(always)]
    fn select(select: Self::Raw, reduced: Self::Raw, original: Self::Raw) -> Self::Raw {
        unsafe { _mm_blendv_epi8(reduced, original, select) }
    }

    #[inline(always)]
    fn from_u32x4(value: u32x4<Avx2>) -> Self::Raw {
        value.into()
    }

    #[inline(always)]
    fn to_u32x4(simd: Avx2, value: Self::Raw) -> u32x4<Avx2> {
        u32x4::simd_from(simd, value)
    }
}

type Context<F, const L: usize, const LOW: usize, const K: usize, const W: usize> =
    UnsaturatedContext<F, Avx2, L, LOW, K, W>;

impl<F: MontgomeryField, const L: usize, const LOW: usize, const K: usize, const W: usize>
    Context<F, L, LOW, K, W>
{
    #[inline(always)]
    fn product<const SQUARE: bool>(
        &self,
        a: &UnsaturatedPacked<F, Avx2, K>,
        b: &UnsaturatedPacked<F, Avx2, K>,
    ) -> UnsaturatedPacked<F, Avx2, K> {
        // The Avx2 token is created only by runtime feature detection. The kernel
        // has its own target-feature boundary, including when called outside vectorize.
        unsafe {
            let aa = a.limbs.map(Into::into);
            let bb;
            let b = if SQUARE {
                &aa
            } else {
                bb = b.limbs.map(Into::into);
                &bb
            };
            let raw = multiply::<K, W, SQUARE>(&aa, b, self.modulus_digits[K - 1] + 1);
            UnsaturatedPacked {
                limbs: raw.map(|v| u32x4::simd_from(self.simd, v)),
                field: PhantomData,
            }
        }
    }
}

impl<F: MontgomeryField, const L: usize, const LOW: usize, const K: usize, const W: usize>
    PackedField for Context<F, L, LOW, K, W>
{
    type Scalar = F;
    type Packed = UnsaturatedPacked<F, Avx2, K>;
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
            || self.unpack_canonical(&self.reduce_with::<Avx2Ops>(value, &self.modulus_digits)),
        )
    }
    fn add(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        self.simd.vectorize(
            #[inline(always)]
            || self.add_mod_with::<Avx2Ops>(a, b),
        )
    }
    fn sub(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        self.simd.vectorize(
            #[inline(always)]
            || self.sub_mod_with::<Avx2Ops>(a, b),
        )
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
        self.simd.vectorize(
            #[inline(always)]
            || self.gather_values::<I0, I1, I2, I3>(values),
        )
    }
    fn butterfly<const BIT: usize>(&self, value: &Self::Packed) -> Self::Packed {
        assert!(BIT == 1 || BIT == 2);
        self.simd.vectorize(
            #[inline(always)]
            || unsafe {
                let mut swapped = self.zero();
                for i in 0..K {
                    let v = value.limbs[i].into();
                    let s = if BIT == 1 {
                        _mm_shuffle_epi32::<0xb1>(v)
                    } else {
                        _mm_shuffle_epi32::<0x4e>(v)
                    };
                    swapped.limbs[i] = u32x4::simd_from(self.simd, s);
                }
                let sum = self.add_mod_with::<Avx2Ops>(value, &swapped);
                let difference = self.sub_mod_with::<Avx2Ops>(&swapped, value);
                UnsaturatedPacked {
                    limbs: core::array::from_fn(|i| {
                        let s = sum.limbs[i].into();
                        let d = difference.limbs[i].into();
                        u32x4::simd_from(
                            self.simd,
                            if BIT == 1 {
                                _mm_blend_epi32::<0xa>(s, d)
                            } else {
                                _mm_blend_epi32::<0xc>(s, d)
                            },
                        )
                    }),
                    field: PhantomData,
                }
            },
        )
    }
}
