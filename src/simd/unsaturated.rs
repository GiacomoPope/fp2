//! Four-way finite-field arithmetic using an unsaturated Montgomery radix.
//!
//! The representation stores each value as `K` normalized limbs in radix
//! `B = 2^W`. Four values are transposed across lanes, so a packed operation
//! processes one limb from each value together. The construction follows the
//! unsaturated-limb approach in *SQISign on ARM*.
//!
//! # Representation and supported layouts
//!
//! A supported modulus has the sparse form `p = d * B^(K - 1) - 1`. Thus `p + 1`
//! has one nonzero limb, which makes each Montgomery reduction column require
//! one correction by `d`. The context validates the modulus shape, limb count,
//! and Montgomery headroom; a matching size alone is not sufficient.
//!
//! The dispatch table recognizes these layouts, with `L` counting scalar
//! radix-2^32 digits:
//!
//! | Scalar digits `L` | Unsaturated limbs `K` | Bits per limb `W` | Field layout |
//! | ---: | ---: | ---: | --- |
//! | 10 | 11 | 30 | MIKE-I default |
//! | 12 | 14 | 28 | MIKE-I conservative |
//! | 16 | 17 | 29 | MIKE-III default |
//! | 18 | 20 | 29 | MIKE-III conservative |
//! | 20 | 24 | 27 | MIKE-V default |
//! | 24 | 27 | 29 | MIKE-V conservative |
//!
//! # Column multiplication and sparse Montgomery reduction
//!
//! Multiplication accumulates each Comba column `sum(a[i] * b[j])` for which
//! `i + j` is the column index, then extracts its digit and carry. Because `p`
//! is congruent to `-1` modulo `B`, the next Montgomery quotient digit is
//! `q = column & (B - 1)`. Adding `q * p` clears that low digit and contributes
//! one product `q * d` to the column `K - 1` positions later. Quotient digits
//! remain available until their corrections are consumed, while the carry moves
//! to the next column; no separate full product array is needed.
//!
//! Squaring uses the symmetry of the product: each diagonal occurs once, and
//! each off-diagonal product occurs once and is doubled. The fixed column and
//! carry schedule is instantiated by the kernels in `neon::unrolled` and
//! `avx2::unrolled`.
//!
//! # Montgomery-domain conversion
//!
//! The scalar field uses Montgomery radix `R_scalar = 2^(64 * N)`, where `N`
//! is its 64-bit word count. This representation uses `R_unsaturated = B^K`.
//! Packing multiplies the scalar value by `R_unsaturated / R_scalar` before
//! transposing its digits. Unpacking canonicalizes the unsaturated value,
//! transcodes the digits, and applies the inverse factor. These factors are
//! powers of two, built with scalar doubling and halving rather than a general
//! field inversion.
//!
//! Conversion adds pack/unpack boundary overhead, so long computations should
//! remain packed rather than repeatedly crossing the conversion boundary.
//!
//! # Bounds and delayed reduction
//!
//! Public operations maintain normalized radix-`B` limbs and an integer in
//! `[0, 2p)`. The constructor requires `p < R_unsaturated / 4`. With inputs
//! below `2p` and a Montgomery quotient below `R_unsaturated`, multiplication
//! satisfies `(a * b + q * p) / R_unsaturated < 4p^2 / R_unsaturated + p < 2p`.
//! Consequently the kernel needs no final canonical subtraction. Addition
//! reduces modulo `2p`, subtraction conditionally adds `2p`, and unpacking
//! reduces below `p` before scalar conversion.
//!
//! Normalized limbs also bound the column accumulators. For the supported
//! layouts, product sums, the sparse correction, and incoming carry fit in
//! 64-bit accumulators: `K <= 28, W <= 29`, or `K = 11, W = 30`. For the
//! 30-bit layout, a conservative bound is fewer than 13 terms of size `2^60`,
//! below `2^64`. Normalization is preserved after every operation because the
//! packed API permits arbitrary operation chains; the bound must not be reused
//! with larger lazy inputs without a new accumulator proof.
//!
//! # Dispatch
//!
//! This module only selects a supported unsaturated architecture and layout.
//! Unsupported levels, moduli, and layouts return the operation to the
//! top-level dispatcher, which selects the radix-32 context or serial arithmetic.
//!
//! [`PackedField`]: crate::packed::PackedField

#![cfg_attr(
    not(any(target_arch = "aarch64", target_arch = "x86_64")),
    allow(dead_code)
)]

use core::marker::PhantomData;

use fearless_simd::{Simd, prelude::*, u32x4};

use super::{MontgomeryDigits, MontgomeryField};

#[cfg(target_arch = "x86_64")]
mod avx2;
#[cfg(target_arch = "aarch64")]
mod neon;

#[cfg(test)]
mod tests;

const MAX_LIMBS: usize = 28;

/// The supported scalar-digit to unsaturated `(limbs, bits)` layouts.
#[cfg_attr(
    not(any(target_arch = "aarch64", target_arch = "x86_64")),
    allow(unused_macros)
)]
macro_rules! for_each_layout {
    ($callback:ident) => {
        $callback! {
            10 => (11, 30), // MIKE-I, fast
            12 => (14, 28), // MIKE-I, conservative
            16 => (17, 29), // MIKE-III, fast
            18 => (20, 29), // MIKE-III, conservative
            20 => (24, 27), // MIKE-V, fast
            24 => (27, 29), // MIKE-V, conservative
        }
    };
}

/// Selects a supported unsaturated architecture and layout.
///
/// `SCALAR_DIGITS` counts scalar radix-2^32 digits, not unsaturated limbs. The
/// operation is returned unchanged when this backend cannot handle the level,
/// modulus, or layout; the top-level dispatcher owns all fallback decisions.
#[inline]
pub(super) fn dispatch<
    F: MontgomeryField,
    O: crate::packed::PackedOperation<F>,
    const SCALAR_DIGITS: usize,
    const LOW_ONES: usize,
>(
    level: fearless_simd::Level,
    operation: O,
) -> Result<O::Output, O> {
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    let _ = level;
    #[cfg(target_arch = "aarch64")]
    if let fearless_simd::Level::Neon(neon) = level {
        // Instantiate each kernel with compile-time constants. The operation is
        // consumed only after the architecture, layout, and modulus validate.
        macro_rules! select_layout {
            ($($scalar_digits:literal => ($limbs:literal, $bits:literal)),* $(,)?) => {
                match SCALAR_DIGITS {
                    $(
                        $scalar_digits => {
                            if let Some(context) = UnsaturatedContext::<
                                F, _, SCALAR_DIGITS, LOW_ONES, $limbs, $bits,
                            >::new(neon) {
                                return Ok(operation.run(context));
                            }
                        }
                    )*
                    _ => {}
                }
            };
        }
        for_each_layout!(select_layout);
    }
    #[cfg(target_arch = "x86_64")]
    if let Some(avx2) = level.as_avx2() {
        macro_rules! select_layout {
            ($($scalar_digits:literal => ($limbs:literal, $bits:literal)),* $(,)?) => {
                match SCALAR_DIGITS {
                    $( $scalar_digits => {
                        if let Some(context) = UnsaturatedContext::<F, _, SCALAR_DIGITS, LOW_ONES, $limbs, $bits>::new(avx2) {
                            return Ok(operation.run(context));
                        }
                    } )*
                    _ => {}
                }
            };
        }
        for_each_layout!(select_layout);
    }
    Err(operation)
}

/// Four Montgomery integers in `[0, 2p)` with normalized radix-2^W limbs.
/// No reduction decision depends on secret control flow.
#[derive(Clone, Copy)]
pub(super) struct UnsaturatedPacked<F: MontgomeryField, S: Simd, const K: usize> {
    pub(super) limbs: [u32x4<S>; K],
    pub(super) field: PhantomData<F>,
}

/// Architecture-neutral state for an unsaturated-radix packed context.
#[derive(Clone, Copy)]
pub(super) struct UnsaturatedContext<
    F: MontgomeryField,
    S: Simd,
    const L: usize,
    const LOW: usize,
    const K: usize,
    const W: usize,
> {
    pub(super) simd: S,
    pub(super) modulus_digits: [u32; K],
    pub(super) twice_modulus_digits: [u32; K],
    /// Scalar field factors bridging the scalar and unsaturated Montgomery radices.
    pub(super) pack_factor: F,
    pub(super) unpack_factor: F,
}

/// Raw vector operations used by the shared normalized-limb loops.
///
/// Implementations stay in the architecture modules so these methods lower
/// directly to the same target intrinsics as the architecture-specific loops.
trait LimbOps<S: Simd> {
    type Raw: Copy;

    fn zero() -> Self::Raw;
    fn splat(value: u32) -> Self::Raw;
    fn add(a: Self::Raw, b: Self::Raw) -> Self::Raw;
    fn sub(a: Self::Raw, b: Self::Raw) -> Self::Raw;
    fn and(a: Self::Raw, b: Self::Raw) -> Self::Raw;
    fn shift_right<const W: usize>(value: Self::Raw) -> Self::Raw;
    fn borrow(value: Self::Raw) -> Self::Raw;
    fn reduction_select(borrow: Self::Raw) -> Self::Raw;
    fn select(select: Self::Raw, reduced: Self::Raw, original: Self::Raw) -> Self::Raw;
    fn from_u32x4(value: u32x4<S>) -> Self::Raw;
    fn to_u32x4(simd: S, value: Self::Raw) -> u32x4<S>;
}

impl<F: MontgomeryField, S: Simd, const L: usize, const LOW: usize, const K: usize, const W: usize>
    UnsaturatedContext<F, S, L, LOW, K, W>
{
    /// Validates the modulus and prepares the shared representation state.
    pub(super) fn new(simd: S) -> Option<Self> {
        if K == 0 || K > MAX_LIMBS || !(27..=30).contains(&W) || (W == 30 && K != 11) {
            return None;
        }
        let words = F::MODULUS;
        if L == 0 || L != 2 * words.len() {
            return None;
        }
        let mut scalar_modulus = [0u32; L];
        for (i, &word) in words.iter().enumerate() {
            scalar_modulus[2 * i] = word as u32;
            scalar_modulus[2 * i + 1] = (word >> 32) as u32;
        }
        if scalar_modulus[0] != u32::MAX
            || scalar_modulus[L - 1] == 0
            || scalar_modulus[L - 1] >> 30 != 0
        {
            return None;
        }
        let low_ones = scalar_modulus
            .iter()
            .take_while(|&&x| x == u32::MAX)
            .count();
        if LOW != 0 && LOW != low_ones {
            return None;
        }
        let top = *words.last()?;
        let bits = 64 * (words.len() - 1) + (64 - top.leading_zeros() as usize);
        if W * K < bits + 2 {
            return None;
        }

        let modulus_digits = scalar_to_unsaturated_digits::<L, K, W>(&scalar_modulus);
        let mask = (1u32 << W) - 1;
        if modulus_digits[0] != mask
            || modulus_digits[..K - 1].iter().any(|&x| x != mask)
            || modulus_digits[K - 1] == mask
        {
            // This context requires the sparse p+1 shape:
            // p + 1 = d * B^(K-1).
            return None;
        }

        // Powers of two convert the scalar Montgomery integer to the
        // unsaturated Montgomery radix; no general field inversion is needed.
        let mut pack_factor = F::ONE;
        let mut unpack_factor = F::ONE;
        for _ in 0..(W * K).abs_diff(32 * L) {
            if W * K >= 32 * L {
                pack_factor = pack_factor.mul2();
                unpack_factor = unpack_factor.half();
            } else {
                pack_factor = pack_factor.half();
                unpack_factor = unpack_factor.mul2();
            }
        }

        Some(Self {
            simd,
            modulus_digits,
            twice_modulus_digits: core::array::from_fn(|i| {
                if i == 0 {
                    mask - 1
                } else if i == K - 1 {
                    2 * modulus_digits[i] + 1
                } else {
                    mask
                }
            }),
            pack_factor,
            unpack_factor,
        })
    }

    #[inline(always)]
    fn zero(&self) -> UnsaturatedPacked<F, S, K> {
        UnsaturatedPacked {
            limbs: [u32x4::splat(self.simd, 0); K],
            field: PhantomData,
        }
    }

    #[inline(always)]
    fn reduce_with<O: LimbOps<S>>(
        &self,
        value: &UnsaturatedPacked<F, S, K>,
        modulus: &[u32; K],
    ) -> UnsaturatedPacked<F, S, K> {
        let mask = O::splat((1u32 << W) - 1);
        let mut borrow = O::zero();
        let mut diff = self.zero();
        for (i, modulus_limb) in modulus.iter().enumerate() {
            let digit = O::from_u32x4(value.limbs[i]);
            let z = O::sub(O::sub(digit, O::splat(*modulus_limb)), borrow);
            diff.limbs[i] = O::to_u32x4(self.simd, O::and(z, mask));
            borrow = O::borrow(z);
        }
        let select = O::reduction_select(borrow);
        for i in 0..K {
            diff.limbs[i] = O::to_u32x4(
                self.simd,
                O::select(
                    select,
                    O::from_u32x4(diff.limbs[i]),
                    O::from_u32x4(value.limbs[i]),
                ),
            );
        }
        diff
    }

    #[inline(always)]
    fn add_mod_with<O: LimbOps<S>>(
        &self,
        a: &UnsaturatedPacked<F, S, K>,
        b: &UnsaturatedPacked<F, S, K>,
    ) -> UnsaturatedPacked<F, S, K> {
        let mask = O::splat((1u32 << W) - 1);
        let mut carry = O::zero();
        let mut sum = self.zero();
        for i in 0..K {
            let av = O::from_u32x4(a.limbs[i]);
            let bv = O::from_u32x4(b.limbs[i]);
            let z = O::add(O::add(av, bv), carry);
            sum.limbs[i] = O::to_u32x4(self.simd, O::and(z, mask));
            carry = O::shift_right::<W>(z);
        }
        self.reduce_with::<O>(&sum, &self.twice_modulus_digits)
    }

    #[inline(always)]
    fn sub_mod_with<O: LimbOps<S>>(
        &self,
        a: &UnsaturatedPacked<F, S, K>,
        b: &UnsaturatedPacked<F, S, K>,
    ) -> UnsaturatedPacked<F, S, K> {
        let mask = O::splat((1u32 << W) - 1);
        let mut borrow = O::zero();
        let mut result = self.zero();
        for i in 0..K {
            let av = O::from_u32x4(a.limbs[i]);
            let bv = O::from_u32x4(b.limbs[i]);
            let z = O::sub(O::sub(av, bv), borrow);
            result.limbs[i] = O::to_u32x4(self.simd, O::and(z, mask));
            borrow = O::borrow(z);
        }
        let select = O::sub(O::zero(), borrow);
        let mut carry = O::zero();
        for i in 0..K {
            let digit = O::from_u32x4(result.limbs[i]);
            let p = O::and(O::splat(self.twice_modulus_digits[i]), select);
            let z = O::add(O::add(digit, p), carry);
            result.limbs[i] = O::to_u32x4(self.simd, O::and(z, mask));
            carry = O::shift_right::<W>(z);
        }
        result
    }

    #[inline(always)]
    fn pack_values(&self, values: [F; 4]) -> UnsaturatedPacked<F, S, K> {
        let adjusted = values.map(|value| value * self.pack_factor);
        let mut scalar_digits = [[0u32; 4]; L];
        for (lane, value) in adjusted.into_iter().enumerate() {
            let mut digits = [0u32; L];
            value.write_montgomery_digits(&mut digits);
            for (word, &digit) in digits.iter().enumerate() {
                scalar_digits[word][lane] = digit;
            }
        }
        scalar_to_unsaturated::<F, S, L, K, W>(self.simd, &scalar_digits)
    }

    #[inline(always)]
    fn unpack_canonical(&self, value: &UnsaturatedPacked<F, S, K>) -> [F; 4] {
        let scalar_digits = unsaturated_to_scalar::<F, S, L, K, W>(value);
        core::array::from_fn(|lane| {
            let digits: [u32; L] = core::array::from_fn(|i| scalar_digits[i][lane]);
            F::from_montgomery_digits(MontgomeryDigits {
                digits: &digits,
                field: PhantomData,
            }) * self.unpack_factor
        })
    }

    #[inline(always)]
    fn gather_values<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        &self,
        values: &[UnsaturatedPacked<F, S, K>],
    ) -> UnsaturatedPacked<F, S, K> {
        assert!(I0 < 4 * values.len());
        assert!(I1 < 4 * values.len());
        assert!(I2 < 4 * values.len());
        assert!(I3 < 4 * values.len());
        UnsaturatedPacked {
            limbs: core::array::from_fn(|i| {
                let a: [u32; 4] = values[I0 / 4].limbs[i].into();
                let b: [u32; 4] = values[I1 / 4].limbs[i].into();
                let c: [u32; 4] = values[I2 / 4].limbs[i].into();
                let d: [u32; 4] = values[I3 / 4].limbs[i].into();
                u32x4::simd_from(self.simd, [a[I0 % 4], b[I1 % 4], c[I2 % 4], d[I3 % 4]])
            }),
            field: PhantomData,
        }
    }
}

#[inline(always)]
pub(super) fn scalar_to_unsaturated_digits<const L: usize, const K: usize, const W: usize>(
    scalar: &[u32; L],
) -> [u32; K] {
    core::array::from_fn(|i| {
        let bit = i * W;
        let word = bit / 32;
        let shift = bit % 32;
        if word >= L {
            return 0;
        }
        let mut value = u64::from(scalar[word] >> shift);
        if shift != 0 && word + 1 < L {
            value |= u64::from(scalar[word + 1]) << (32 - shift);
        }
        value as u32 & ((1u32 << W) - 1)
    })
}

#[inline(always)]
pub(super) fn scalar_to_unsaturated<
    F: MontgomeryField,
    S: Simd,
    const L: usize,
    const K: usize,
    const W: usize,
>(
    simd: S,
    scalar: &[[u32; 4]; L],
) -> UnsaturatedPacked<F, S, K> {
    let limbs: [[u32; 4]; K] = core::array::from_fn(|i| {
        core::array::from_fn(|lane| {
            let digits = core::array::from_fn(|j| scalar[j][lane]);
            scalar_to_unsaturated_digits::<L, K, W>(&digits)[i]
        })
    });
    UnsaturatedPacked {
        limbs: limbs.map(|x| u32x4::simd_from(simd, x)),
        field: PhantomData,
    }
}

#[inline(always)]
pub(super) fn unsaturated_to_scalar<
    F: MontgomeryField,
    S: Simd,
    const L: usize,
    const K: usize,
    const W: usize,
>(
    value: &UnsaturatedPacked<F, S, K>,
) -> [[u32; 4]; L] {
    let limbs: [[u32; 4]; K] = value.limbs.map(Into::into);
    core::array::from_fn(|word| {
        core::array::from_fn(|lane| {
            let bit = word * 32;
            let limb = bit / W;
            let shift = bit % W;
            if limb >= K {
                return 0;
            }
            let mut value = u64::from(limbs[limb][lane] >> shift);
            if limb + 1 < K {
                value |= u64::from(limbs[limb + 1][lane]) << (W - shift);
            }
            if 2 * W - shift < 32 && limb + 2 < K {
                value |= u64::from(limbs[limb + 2][lane]) << (2 * W - shift);
            }
            value as u32
        })
    })
}
