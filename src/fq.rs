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

    // Specialised arithmetic operations
    fn set_mul_small(&mut self, k: i32);
    fn square(self) -> Self;
    fn set_invert(&mut self);
    fn sqrt(self) -> (Self, u32);
    fn fourth_root(self) -> (Self, u32);

    // Multiplication by constants
    fn set_mul2(&mut self);
    fn set_mul3(&mut self);
    fn set_mul4(&mut self);
    fn set_mul8(&mut self);
    fn set_half(&mut self);
    fn half(self) -> Self;
    fn mul2(self) -> Self;
    fn mul3(self) -> Self;
    fn mul4(self) -> Self;
    fn mul8(self) -> Self;

    // Compute the legendre symbol (a/p)
    fn legendre(self) -> i32;

    // Specialised operations for Fp2 arithmetic
    fn sum_of_products(a1: &Self, b1: &Self, a2: &Self, b2: &Self) -> Self;
    fn difference_of_products(a1: &Self, b1: &Self, a2: &Self, b2: &Self) -> Self;

    // Constant time manipulations
    fn select(a: &Self, b: &Self, ctl: u32) -> Self;
    fn set_select(&mut self, a: &Self, b: &Self, ctl: u32);
    fn set_cond(&mut self, rhs: &Self, ctl: u32);
    fn set_condneg(&mut self, ctl: u32);
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
