//! Trait for Finite Field Arithmetic for GF(p^2) with modulus x^2 + 1.

use super::Fp;

/// Trait for Finite field arithmetic for the extension field GF(p^2) with modulus x^2 + 1.
pub trait Fp2: Fp {
    /// Predefined constant element representing the value 0 + i such that
    /// i^2 = -1, a fourth-root of unity.
    const ZETA: Self;

    /// Predefined constant element representing the value 0 - i such that
    /// i^2 = -1, a fourth-root of unity.
    const MINUS_ZETA: Self;

    /// Set the "real" component of self to an integer of type `i32` in place.
    fn set_x0_small(&mut self, x: i32);

    /// Set the "imaginary" component of self to an integer of type `i32` in place.
    fn set_x1_small(&mut self, x: i32);

    /// Return the value x0 + i*x1 for a given two integers of type `i32`.
    fn from_i32_pair(x0: i32, x1: i32) -> Self;

    /// Return the value x0 + i*x1 for a given two integers of type `u32`.
    fn from_u32_pair(x0: u32, x1: u32) -> Self;

    /// Return the value x0 + i*x1 for a given two integers of type `i64`.
    fn from_i64_pair(x0: i64, x1: i64) -> Self;

    /// Return the value x0 + i*x1 for a given two integers of type `u64`.
    fn from_u64_pair(x0: u64, x1: u64) -> Self;

    /// Negate the imaginary pary of this value
    fn set_conjugate(&mut self);

    /// Compute the complex conjugate of the value a + i*b, i.e. a - i*b.
    fn conjugate(self) -> Self;

    /// Return `0xFFFFFFFF` when this value is a square in GF(p) and
    /// `0x00000000` otherwise.
    fn is_square_base_field(self) -> u32;

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
}
