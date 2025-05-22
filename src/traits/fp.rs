//! Trait for Finite Field Arithmetic for the field GF(p).

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Display;

/// Trait for Finite Field Arithmetic for the field GF(p).
pub trait Fp:
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

    /// Predefined constant element representing the value 0.
    const ZERO: Self;

    /// Predefined constant element representing the value 1.
    const ONE: Self;

    /// Predefined constant element representing the value 2.
    const TWO: Self;

    /// Predefined constant element representing the value 3.
    const THREE: Self;

    /// Predefined constant element representing the value 4.
    const FOUR: Self;

    /// Predefined constant element representing the value -1.
    const MINUS_ONE: Self;

    /// Return the value x for a given integer x of type `i32`.
    fn from_i32(x: i32) -> Self;

    /// Return the value x for a given integer x of type `u32`.
    fn from_u32(x: u32) -> Self;

    /// Return the value x for a given integer x of type `i64`.
    fn from_i64(x: i64) -> Self;

    /// Return the value x for a given integer x of type `u64`.
    fn from_u64(x: u64) -> Self;

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

    /// Return `0xFFFFFFFF` when this value is a square in GF(p^2) and
    /// `0x00000000` otherwise.
    fn is_square(self) -> u32;

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

    /// Get the "hash" of the value (low 64 bits of the Montgomery representation)
    fn hashcode(self) -> u64;
}
