use core::marker::PhantomData;

use fearless_simd::{Simd, prelude::*, u64x4};

use crate::packed::PackedField;
use crate::traits::Fq;

use super::{MontgomeryDigits, MontgomeryField};

const DIGIT_MASK: u64 = 0xffff_ffff;

/// Four field elements in radix-2^32 Montgomery representation.
///
/// Each lane is in [0,2p) with normalized radix-32 digits; unpacking reduces below p.
#[derive(Clone, Copy)]
pub(super) struct Packed<F: Fq, S: Simd, const L: usize> {
    pub(super) limbs: [u64x4<S>; L],
    pub(super) field: PhantomData<F>,
}

/// An opaque packed arithmetic context tied to a scalar field and SIMD backend.
#[derive(Clone, Copy)]
pub(super) struct Context<F: Fq, S: Simd, const L: usize, const LOW: usize = 0> {
    simd: S,
    modulus: Packed<F, S, L>,
    pub(super) twice_modulus: Packed<F, S, L>,
    low_ones: usize,
}

impl<F: MontgomeryField, S: Simd, const L: usize, const LOW: usize> Context<F, S, L, LOW> {
    /// Constructs a context for a supported modulus, or returns `None`.
    ///
    /// The limb count `L` must match the scalar radix, `p[0]` must be `-1`, the
    /// top digit must be nonzero, and `p` must be less than `2^(32L)/4`.
    pub(super) fn new(simd: S) -> Option<Self> {
        let words = F::MODULUS;
        if L == 0 || L != 2 * words.len() {
            return None;
        }
        let mut modulus = [0u32; L];
        for (i, &word) in words.iter().enumerate() {
            modulus[2 * i] = word as u32;
            modulus[2 * i + 1] = (word >> 32) as u32;
        }
        if modulus[0] != u32::MAX || modulus[L - 1] == 0 || modulus[L - 1] >> 30 != 0 {
            return None;
        }
        let low_ones = modulus.iter().take_while(|&&x| x == u32::MAX).count();
        if LOW != 0 && LOW != low_ones {
            return None;
        }
        let mut carry = 0u64;
        let twice_modulus = modulus.map(|x| {
            let z = 2 * u64::from(x) + carry;
            carry = z >> 32;
            z as u32
        });
        Some(simd.vectorize(
            #[inline(always)]
            || Self {
                simd,
                modulus: splat_digits(simd, &modulus),
                twice_modulus: splat_digits(simd, &twice_modulus),
                low_ones,
            },
        ))
    }

    /// Multiplies four lane pairs within a non-inlined vectorization boundary.
    #[inline(never)]
    fn mul_outlined(&self, a: &Packed<F, S, L>, b: &Packed<F, S, L>) -> Packed<F, S, L> {
        self.simd.vectorize(
            #[inline(always)]
            || self.montgomery_mul(a, b),
        )
    }

    /// Multiplies using coarsely integrated operand scanning Montgomery reduction.
    ///
    /// For `B = 2^32` and `R = B^L`, each product/carry accumulation is at most
    /// `(B-1)^2 + 2*(B-1) = 2^64-1`. The recurrence maintains `T < a+p < 3p < R`,
    /// so discarding the top carry is valid. The final result is `(ab+qp)/R` with
    /// `q < R`, hence below `4p^2/R+p < 2p` because `p < R/4`.
    #[inline(always)]
    fn montgomery_mul(&self, a: &Packed<F, S, L>, b: &Packed<F, S, L>) -> Packed<F, S, L> {
        let zero = u64x4::splat(self.simd, 0);
        let mut t = [zero; L];
        for i in 0..L {
            let bi = b.limbs[i] & DIGIT_MASK;
            let z = (a.limbs[0] & DIGIT_MASK) * bi + t[0];
            let m = z & DIGIT_MASK;
            let mut product_carry = z >> 32;
            let low_ones = if LOW == 0 { self.low_ones } else { LOW };
            // For p[j]=B-1, reduction carry stays m and the normalized product
            // digit is unchanged. Write directly into its shifted destination.
            for j in 1..low_ones {
                let z = (a.limbs[j] & DIGIT_MASK) * bi + t[j] + product_carry;
                product_carry = z >> 32;
                t[j - 1] = z & DIGIT_MASK;
            }
            let mut reduction_carry = m;
            for j in low_ones..L {
                let product = (a.limbs[j] & DIGIT_MASK) * bi + t[j] + product_carry;
                product_carry = product >> 32;
                let reduced = (product & DIGIT_MASK)
                    + m * (self.modulus.limbs[j] & DIGIT_MASK)
                    + reduction_carry;
                t[j - 1] = reduced & DIGIT_MASK;
                reduction_carry = reduced >> 32;
            }
            t[L - 1] = (product_carry + reduction_carry) & DIGIT_MASK;
        }
        Packed {
            limbs: t,
            field: PhantomData,
        }
    }

    /// Reduces each lane from `[0, 2p)` to `[0, p)` by subtracting `p` if needed.
    #[inline(always)]
    fn reduce_once(&self, value: &Packed<F, S, L>) -> Packed<F, S, L> {
        self.reduce_with(value, &self.modulus)
    }

    /// Conditionally subtracts the supplied modulus from each lane.
    ///
    /// Each input lane must be below twice the corresponding modulus lane.
    /// Modulus digits are normalized radix-32 integers, not Montgomery values.
    #[inline(always)]
    fn reduce_with(&self, value: &Packed<F, S, L>, modulus: &Packed<F, S, L>) -> Packed<F, S, L> {
        let zero = u64x4::splat(self.simd, 0);
        let one = u64x4::splat(self.simd, 1);
        let mut borrow = zero;
        let mut difference = [zero; L];
        for (i, digit) in difference.iter_mut().enumerate() {
            let z = value.limbs[i] - (modulus.limbs[i] + borrow);
            *digit = z & DIGIT_MASK;
            borrow = z >> 63;
        }
        let use_difference = borrow - one;
        let mut result = packed_zero(self.simd);
        for (i, digit) in difference.iter().enumerate() {
            result.limbs[i] = (*digit & use_difference) | (value.limbs[i] & !use_difference);
        }
        result
    }
}

impl<F: MontgomeryField, S: Simd, const L: usize, const LOW: usize> PackedField
    for Context<F, S, L, LOW>
{
    type Scalar = F;
    type Packed = Packed<F, S, L>;

    #[inline(always)]
    fn is_accelerated(&self) -> bool {
        !self.simd.level().is_fallback()
    }

    #[inline(always)]
    fn vectorize<R>(&self, f: impl FnOnce() -> R) -> R {
        self.simd.vectorize(f)
    }

    #[inline(always)]
    fn pack(&self, values: [F; 4]) -> Self::Packed {
        self.simd.vectorize(
            #[inline(always)]
            || {
                let mut digits = [[0u32; L]; 4];
                for lane in 0..4 {
                    values[lane].write_montgomery_digits(&mut digits[lane]);
                }
                Packed {
                    limbs: core::array::from_fn(|i| {
                        u64x4::simd_from(
                            self.simd,
                            core::array::from_fn(|lane| u64::from(digits[lane][i])),
                        )
                    }),
                    field: PhantomData,
                }
            },
        )
    }

    #[inline(always)]
    fn unpack(&self, value: &Self::Packed) -> [F; 4] {
        self.simd.vectorize(
            #[inline(always)]
            || {
                let canonical = self.reduce_once(value);
                let digits: [[u64; 4]; L] = canonical.limbs.map(Into::into);
                core::array::from_fn(|lane| {
                    let digits: [u32; L] = core::array::from_fn(|i| digits[i][lane] as u32);
                    F::from_montgomery_digits(MontgomeryDigits {
                        digits: &digits,
                        field: PhantomData,
                    })
                })
            },
        )
    }

    #[inline(always)]
    fn add(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        let zero = u64x4::splat(self.simd, 0);
        let mut carry = zero;
        let mut sum = packed_zero(self.simd);
        for i in 0..L {
            let z = a.limbs[i] + b.limbs[i] + carry;
            sum.limbs[i] = z & DIGIT_MASK;
            carry = z >> 32;
        }
        self.reduce_with(&sum, &self.twice_modulus)
    }

    #[inline(always)]
    fn sub(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        let zero = u64x4::splat(self.simd, 0);
        let mut borrow = zero;
        let mut difference: Packed<F, S, L> = packed_zero(self.simd);
        for i in 0..L {
            let z = a.limbs[i] - (b.limbs[i] + borrow);
            difference.limbs[i] = z & DIGIT_MASK;
            borrow = z >> 63;
        }
        let add_mask = zero - borrow;
        let mut carry = zero;
        let mut result = packed_zero(self.simd);
        for i in 0..L {
            let z = difference.limbs[i] + (self.twice_modulus.limbs[i] & add_mask) + carry;
            result.limbs[i] = z & DIGIT_MASK;
            carry = z >> 32;
        }
        result
    }

    #[inline(always)]
    fn mul(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed {
        if L <= 12 {
            self.montgomery_mul(a, b)
        } else {
            self.mul_outlined(a, b)
        }
    }

    #[inline(always)]
    fn square(&self, a: &Self::Packed) -> Self::Packed {
        self.mul(a, a)
    }

    #[inline(always)]
    fn gather<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        &self,
        values: &[Self::Packed],
    ) -> Self::Packed {
        assert!(I0 < 4 * values.len());
        assert!(I1 < 4 * values.len());
        assert!(I2 < 4 * values.len());
        assert!(I3 < 4 * values.len());
        Packed {
            limbs: core::array::from_fn(|i| {
                let a: [u64; 4] = values[I0 / 4].limbs[i].into();
                let b: [u64; 4] = values[I1 / 4].limbs[i].into();
                let c: [u64; 4] = values[I2 / 4].limbs[i].into();
                let d: [u64; 4] = values[I3 / 4].limbs[i].into();
                u64x4::simd_from(self.simd, [a[I0 % 4], b[I1 % 4], c[I2 % 4], d[I3 % 4]])
            }),
            field: PhantomData,
        }
    }

    #[inline(always)]
    fn butterfly<const BIT: usize>(&self, a: &Self::Packed) -> Self::Packed {
        let add_mask = match BIT {
            1 => u64x4::simd_from(self.simd, [u64::MAX, 0, u64::MAX, 0]),
            2 => u64x4::simd_from(self.simd, [u64::MAX, u64::MAX, 0, 0]),
            _ => panic!("packed four-lane butterflies only support bits one and two"),
        };
        let sub_mask = !add_mask;
        let zero = u64x4::splat(self.simd, 0);
        let mut carry = sub_mask & 1u64;
        let mut value = packed_zero(self.simd);
        for i in 0..L {
            let digit = (a.limbs[i] ^ sub_mask) & DIGIT_MASK;
            let lanes: [u64; 4] = a.limbs[i].into();
            let shuffled = u64x4::simd_from(
                self.simd,
                [lanes[BIT], lanes[1 ^ BIT], lanes[2 ^ BIT], lanes[3 ^ BIT]],
            );
            let sum = shuffled + digit + carry;
            value.limbs[i] = sum & DIGIT_MASK;
            carry = sum >> 32;
        }
        let add_p = (carry - 1u64) & sub_mask;
        carry = add_mask & 1u64;
        let mut corrected = value;
        for i in 0..L {
            let p = self.twice_modulus.limbs[i] & (add_p | add_mask);
            let digit = (p ^ add_mask) & DIGIT_MASK;
            let sum = value.limbs[i] + digit + carry;
            corrected.limbs[i] = sum & DIGIT_MASK;
            carry = sum >> 32;
        }
        let choose = (zero - carry) | sub_mask;
        for i in 0..L {
            value.limbs[i] = (corrected.limbs[i] & choose) | (value.limbs[i] & !choose);
        }
        value
    }
}

/// Creates a pack with all limbs and lanes set to zero.
#[inline(always)]
fn packed_zero<F: Fq, S: Simd, const L: usize>(simd: S) -> Packed<F, S, L> {
    let zero = u64x4::splat(simd, 0);
    Packed {
        limbs: [zero; L],
        field: PhantomData,
    }
}

/// Copies each radix-32 digit into all four lanes without Montgomery conversion.
#[inline(always)]
fn splat_digits<F: Fq, S: Simd, const L: usize>(simd: S, digits: &[u32; L]) -> Packed<F, S, L> {
    let mut result = packed_zero(simd);
    for (i, digit) in digits.iter().enumerate() {
        result.limbs[i] = u64x4::splat(simd, u64::from(*digit));
    }
    result
}
