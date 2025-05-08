use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
{
    // Constants for internal computations
    const ENCODED_LENGTH: usize;

    // Precomputed elements in the finite field
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const FOUR: Self;
    const MINUS_ONE: Self;
    const ZETA: Self;
    const MINUS_ZETA: Self;

    // Comaprison functions
    fn iszero(self) -> u32;
    fn equals(self, rhs: &Self) -> u32;

    // Negate in-place, kinda like "NegAssign"
    fn set_neg(&mut self);

    // Multiplication by constants
    fn set_half(&mut self);
    fn set_mul2(&mut self);
    fn set_mul3(&mut self);
    fn set_mul4(&mut self);
    fn set_mul8(&mut self);

    fn half(self) -> Self;
    fn mul2(self) -> Self;
    fn mul3(self) -> Self;
    fn mul4(self) -> Self;
    fn mul8(self) -> Self;

    // Specialised arithmetic operations
    fn set_conjugate(&mut self);
    fn set_mul_small(&mut self, k: i32);
    fn set_square(&mut self);
    fn set_invert(&mut self);
    fn set_pow(&mut self, e: &[u8], ebitlen: usize);
    fn set_pow_ext(&mut self, e: &[u8], eoff: usize, ebitlen: usize);
    fn set_pow_u64(&mut self, e: u64, ebitlen: usize);
    fn set_pow_u64_vartime(&mut self, e: u64);

    fn conjugate(self) -> Self;
    fn mul_small(self, k: i32) -> Self;
    fn square(self) -> Self;
    fn invert(self) -> Self;
    fn pow(self, e: &[u8], ebitlen: usize) -> Self;
    fn pow_ext(self, e: &[u8], eoff: usize, ebitlen: usize) -> Self;
    fn pow_u64(&mut self, e: u64, ebitlen: usize) -> Self;
    fn pow_u64_vartime(&mut self, e: u64) -> Self;

    // Square roots and Fourth Roots
    fn set_sqrt(&mut self) -> u32;
    fn set_fourth_root(&mut self) -> u32;
    fn sqrt(self) -> (Self, u32);
    fn fourth_root(self) -> (Self, u32);

    // Compute the legendre symbol (a/p)
    fn legendre(self) -> i32;

    // Batch inversion
    fn batch_invert(xx: &mut [Self]);

    // Constant time manipulations
    fn set_select(&mut self, a: &Self, b: &Self, ctl: u32);
    fn set_cond(&mut self, rhs: &Self, ctl: u32);
    fn set_condneg(&mut self, ctl: u32);
    fn select(a: &Self, b: &Self, ctl: u32) -> Self;

    //
    fn condswap(a: &mut Self, b: &mut Self, ctl: u32);

    // Encoding and Decoding
    fn encode(self) -> [u8; Self::ENCODED_LENGTH];
    fn decode(buf: &[u8]) -> (Self, u32);

    // Randomisation
    fn set_rand<R: ::rand_core::CryptoRng + ::rand_core::RngCore>(&mut self, rng: &mut R);
    fn rand<R: ::rand_core::CryptoRng + ::rand_core::RngCore>(rng: &mut R) -> Self;

    // Utility functions
    fn hashcode(self) -> u64;
}
