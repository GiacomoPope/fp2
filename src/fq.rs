//! A Trait for Finite Field Arithmetic for Fp^2 with modulus x^2 + 1.
//!
//! # Warning
//!
//! This library, and hence this trait are currently only configured to work with the finite field
//! Fp^2, a degree two extension of Fp with modulus x^2 + 1 (hence we require p = 3 mod 4). This is
//! the finite field most used in isogeny-based cryptography, for which this library has been
//! developed for. Accomodating other finite fields to work with this trait is possible and can be
//! done in the future

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Display;

/// A trait for finite field arithmetic for the field Fp^2 with modulus x^2 + 1.
pub trait Fq:
    Copy
    + Neg<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Display
{
    /// The length of the encoded representation of the finite field element.
    const ENCODED_LENGTH: usize;

    /// Predefined constant element representing the value 0 + 0*i.
    const ZERO: Self;

    /// Predefined constant element representing the value 1 + 0*i.
    const ONE: Self;

    /// Predefined constant element representing the value 2 + 0*i.
    const TWO: Self;

    /// Predefined constant element representing the value 3 + 0*i.
    const THREE: Self;

    /// Predefined constant element representing the value 4 + 0*i.
    const FOUR: Self;

    /// Predefined constant element representing the value -1 + 0*i.
    const MINUS_ONE: Self;

    /// Predefined constant element representing the value 0 + i such that
    /// i^2 = -1, a fourth-root of unity.
    const ZETA: Self;

    /// Predefined constant element representing the value 0 - i such that
    /// i^2 = -1, a fourth-root of unity.
    const MINUS_ZETA: Self;

    /// Return `0xFFFFFFFF` if this value is zero, or `0x00000000` otherwise.
    fn is_zero(self) -> u32;

    /// Return `0xFFFFFFFF` if this value is equal to rhs, or `0x00000000`
    /// otherwise.
    fn equals(self, rhs: &Self) -> u32;

    /// Negate this value.
    fn set_neg(&mut self);

    /// Halve this value.
    fn set_half(&mut self);

    /// Double this value.
    fn set_mul2(&mut self);

    /// Triple this value.
    fn set_mul3(&mut self);

    /// Quadruple this value.
    fn set_mul4(&mut self);

    /// Multiply this value by 8
    fn set_mul8(&mut self);

    /// Compute the half of this value.
    fn half(self) -> Self;

    /// Compute the sum of this value with itself.
    fn mul2(self) -> Self;

    /// Compute the triple of this value.
    fn mul3(self) -> Self;

    /// Compute the quadruple of this value.
    fn mul4(self) -> Self;

    /// Compute 8 times this value.
    fn mul8(self) -> Self;

    /// Negate the imaginary pary of this value
    fn set_conjugate(&mut self);

    /// Multiply this value by a small signed integer.
    fn set_mul_small(&mut self, k: i32);

    /// Replace this value with its square.
    fn set_square(&mut self);

    /// Replace this value with its inverse.
    fn set_invert(&mut self);

    /// Raise this value to the power `e`. Exponent `e` is encoded in
    /// unsigned little-endian convention over exactly `ebitlen` bits.
    fn set_pow(&mut self, e: &[u8], ebitlen: usize);

    /// Raise this value to the power e. Exponent e is encoded in
    /// unsigned little-endian convention, over exactly `ebitlen` bits,
    /// and starting at the bit offset eoff.
    fn set_pow_ext(&mut self, e: &[u8], eoff: usize, ebitlen: usize);

    /// Raise this value to the power `e`. The exponent length (in bits)
    /// MUST be at most `ebitlen`. This is constant-time for both the
    /// base value (`self`) and the exponent (`e`); the exponent maximum
    /// size (`ebitlen`) is considered non-secret.
    fn set_pow_u64(&mut self, e: u64, ebitlen: usize);

    /// Raise this value to the power `e`. The exponent is considered
    /// non-secret.
    fn set_pow_u64_vartime(&mut self, e: u64);

    /// Compute the complex conjugate of the value a + i*b, i.e. a - i*b.
    fn conjugate(self) -> Self;

    /// Compute the product of this value by a small (unsigned) integer `k`.
    fn mul_small(self, k: i32) -> Self;

    /// Compute the square of this value.
    fn square(self) -> Self;

    /// Compute the inverse of this value
    fn invert(self) -> Self;

    /// Return this value to the power `e` (as a new element). Exponent `e`
    /// is encoded in unsigned little-endian convention over exactly
    /// `ebitlen` bits.
    fn pow(self, e: &[u8], ebitlen: usize) -> Self;

    /// Return this value to the power `e` (as a new element). Exponent `e`
    /// is encoded in unsigned little-endian convention over exactly
    /// `ebitlen` bits, and starting at the bit offset eoff.
    fn pow_ext(self, e: &[u8], eoff: usize, ebitlen: usize) -> Self;

    /// Return this value to the power `e`. The exponent length (in bits)
    /// MUST be at most `ebitlen`. This is constant-time for both the
    /// base value (`self`) and the exponent (`e`); the exponent maximum
    /// size (`ebitlen`) is considered non-secret.
    fn pow_u64(&mut self, e: u64, ebitlen: usize) -> Self;

    /// Return this value to the power e. The exponent is considered
    /// non-secret.
    fn pow_u64_vartime(&mut self, e: u64) -> Self;

    /// Set this value to its square root. Returned value is `0xFFFFFFFF` if
    /// the operation succeeded (value was indeed a quadratic residue), or
    /// `0x00000000` otherwise. On success, the chosen root is the one whose
    /// least significant bit (as an integer in `[0..p-1]`) is zero. On
    /// failure, this value is set to zero.
    fn set_sqrt(&mut self) -> u32;

    /// Set this value to its fourth root. Returned value is `0xFFFFFFFF` if
    /// the operation succeeded (value was indeed some element to the power of four), or
    /// `0x00000000` otherwise. On success, the chosen root is the one whose
    /// least significant bit (as an integer in `[0..p-1]`) is zero. On
    /// failure, this value is set to zero.
    fn set_fourth_root(&mut self) -> u32;

    /// Compute the square root of this value. If this value is indeed a
    /// quadratic residue, then this returns `(x, 0xFFFFFFFF)`, with `x` being
    /// the (unique) square root of this value whose least significant bit
    /// is zero (when normalized to an integer in `[0..p-1]`). If this value
    /// is not a quadratic residue, then this returns (zero, `0x00000000`).
    fn sqrt(self) -> (Self, u32);

    /// Compute the fourth root of this value. If this value is indeed some
    /// element to the power of four, then this returns `(x, 0xFFFFFFFF)`, with `x` being
    /// the (unique) fourth root of this value whose least significant bit
    /// is zero (when normalized to an integer in `[0..p-1]`). If this value
    /// is not some element to the power of four, then this returns (zero, `0x00000000`).
    fn fourth_root(self) -> (Self, u32);

    /// Legendre symbol on this value. Return value is:
    /// -  0   if this value is zero
    /// - +1   if this value is a non-zero quadratic residue
    /// - -1   if this value is not a quadratic residue
    fn legendre(self) -> i32;

    /// Precompute two vectors of values used to optimally solve the dlog
    /// for elements of order 2^n exactly.
    ///
    /// Explicitly, this involves computing:
    /// - A table dlog_table of indicies corresponding to where to split
    ///   the dlog recursively of type Vec<usize>
    /// - A table of Fp2 elements `gpp[j] = g^(2^dlog_table[j])` of type
    ///   of type `Vec<Self>`
    ///
    /// Note that the first value (`gpp[0]`) is `g` itself, and the last one must
    /// be `-1` (otherwise, `g` does not have order exactly 2^e).
    fn precompute_dlp_tables(self, n: usize) -> (Vec<usize>, Vec<Self>, u32);

    /// Find integer `v` (modulo 2^e) such that `x = self^v`. If self
    /// has order exactly 2^e, and there is a solution v, then this
    /// function returns (v, `0xFFFFFFFF`). If self does not have order
    /// exactly 2^e (including if `self^(2^(e-1)) = 1`, i.e. the order of
    /// self is a strict divisor or 2^e), or if there is no solution,
    /// then this function returns `(0, 0)`.
    ///
    /// Optionally include precomputed values from the method precompute_dlp_tables
    /// otherwise these are computed at runtime.
    fn solve_dlp_2e(
        self,
        x: &Self,
        e: usize,
        precomputed_tables: Option<(&Vec<usize>, &Vec<Self>)>,
    ) -> (Vec<u8>, u32);

    /// Given `n` elements, computes the inverse of all elements in-place at a cost
    /// of one inversion and 3*(n - 1) multiplications using Montgomery's trick
    fn batch_invert(xx: &mut [Self]);

    /// Return `a` or `b`, if `ctl` is `0x00000000` or `0xFFFFFFFF`, respectively.
    /// `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    /// The value of `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    fn set_select(&mut self, a: &Self, b: &Self, ctl: u32);

    /// Set this value to `rhs` if `ctl` is `0xFFFFFFFF`; leave it unchanged if
    /// `ctl` is `0x00000000`.
    /// The value of `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    fn set_cond(&mut self, rhs: &Self, ctl: u32);

    /// Negate this value if `ctl` is `0xFFFFFFFF`; leave it unchanged if
    /// `ctl` is `0x00000000`.
    /// The value of `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    fn set_condneg(&mut self, ctl: u32);

    /// Return `a` or `b`, if `ctl` is `0x00000000` or `0xFFFFFFFF`, respectively.
    /// `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    /// The value of `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    fn select(a: &Self, b: &Self, ctl: u32) -> Self;

    /// Exchange the values of `a` and `b` is `ctl` is `0xFFFFFFFF`; leave both
    /// values unchanged if `ctl` is `0x00000000`.
    /// The value of `ctl` MUST be either `0x00000000` or `0xFFFFFFFF`.
    fn condswap(a: &mut Self, b: &mut Self, ctl: u32);

    /// Encode this value into bytes. Encoding uses little-endian, has
    /// a fixed size (for a given field), and is canonical.
    fn encode(self) -> [u8; Self::ENCODED_LENGTH];

    /// Decode the provided bytes into a field element. Returned values
    /// are the element and `0xFFFFFFFF` on success, or the zero element and
    /// `0x00000000` on failure. A failure is reported if the source slice
    /// does not have exactly the canonical encoding length of a field
    /// element (`Self::ENCODED_LENGTH`), or if the source encodes
    /// an integer which is not in the `[0..(p-1)]` range.
    fn decode(buf: &[u8]) -> (Self, u32);

    /// Decode the provided bytes into a field element. The source slice
    /// can have arbitrary length; the bytes are interpreted with the
    /// unsigned little-endian convention (no sign bit), with the first half
    /// of the bytes corresponding to x0 and the latter half to x1. For each
    /// resulting integer, the result is reduced modulo the field modulus p.
    /// By definition, this function does not enforce canonicality of the source
    /// value.
    fn decode_reduce(buf: &[u8]) -> Self;

    /// Set this structure to a random field element (indistinguishable
    /// from uniform generation).
    fn set_rand<R: ::rand_core::CryptoRng + ::rand_core::RngCore>(&mut self, rng: &mut R);

    /// Return a new random field element (indistinguishable from
    /// uniform generation).
    fn rand<R: ::rand_core::CryptoRng + ::rand_core::RngCore>(rng: &mut R) -> Self;

    /// Get the "hash" of the value. For x = a + i*b, this is:
    ///    `(hashcode(a) << 1) | (hashcode(b) & 1)`
    /// i.e. bit `0` is bit `0` of `b`, and bits 1..63 are bits 0..62 of `a`
    /// (both in Montgomery representation).
    fn hashcode(self) -> u64;
}
