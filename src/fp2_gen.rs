//! A macro to define efficient and constant-time arithmetic for the finite field Fp^2
//! with modulus x^2 + 1
//!
//! # Authorship and History
//!
//! The majority of this code has been adapted from code written by Thomas Pornin
//! from collaboration in previous projects and several methods which appear in other
//! macros in the cryptographic research library crrl <https://github.com/pornin/crrl>
//!
//! This code has also been used in a handful of isogeny-based cryptography research
//! projects before being rewritten for this crate, including:
//! - <https://github.com/ThetaIsogenies/two-isogenies>
//! - <https://github.com/GiacomoPope/cubical-pairings>
//! - <https://github.com/GiacomoPope/ThetaCGL>

/// A macro to define the degree two extension of the finite field Fp, with
/// modulus x^2 + 1. All functions are designed to run in constant time.
///
/// Macro expectations:
/// - A typename for the finite field generated.
/// - A finite field type Fp with p = 3 mod 4, for example one generated with the macro `define_fp_core`.
#[macro_export]
macro_rules! define_fp2_from_type {
    (
        typename = $typename:ident,
        base_field = $Fp:ty,
    ) => {
        /// GF(p^2) implementation.
        #[derive(Clone, Copy, Debug)]
        pub struct $typename {
            x0: $Fp,
            x1: $Fp,
        }

        impl $typename {
            pub const ZERO: Self = Self {
                x0: <$Fp>::ZERO,
                x1: <$Fp>::ZERO,
            };
            pub const ONE: Self = Self {
                x0: <$Fp>::ONE,
                x1: <$Fp>::ZERO,
            };
            pub const TWO: Self = Self {
                x0: <$Fp>::TWO,
                x1: <$Fp>::ZERO,
            };
            pub const THREE: Self = Self {
                x0: <$Fp>::THREE,
                x1: <$Fp>::ZERO,
            };
            pub const FOUR: Self = Self {
                x0: <$Fp>::FOUR,
                x1: <$Fp>::ZERO,
            };
            pub const MINUS_ONE: Self = Self {
                x0: <$Fp>::MINUS_ONE,
                x1: <$Fp>::ZERO,
            };
            pub const ZETA: Self = Self {
                x0: <$Fp>::ZERO,
                x1: <$Fp>::ONE,
            };
            pub const MINUS_ZETA: Self = Self {
                x0: <$Fp>::ZERO,
                x1: <$Fp>::MINUS_ONE,
            };

            pub const ENCODED_LENGTH: usize = 2 * <$Fp>::ENCODED_LENGTH;
            pub const CHAR_BIT_LENGTH: usize = <$Fp>::BIT_LENGTH;

            pub const fn new(re: &$Fp, im: &$Fp) -> Self {
                Self { x0: *re, x1: *im }
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            fn from_i32(x: i32) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_i32(x);
                r
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            fn from_i64(x: i64) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_i64(x);
                r
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            fn from_u32(x: u32) -> Self {
                Self::from_u64(x as u64)
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            fn from_u64(x: u64) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_u64(x);
                r
            }

            /// Create an element [x0, x1] from a pair of integers
            #[inline(always)]
            fn from_u32_pair(x0: u32, x1: u32) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_u32(x0);
                r.x1 = <$Fp>::from_u32(x1);
                r
            }

            /// Create an element [x0, x1] from a pair of integers
            #[inline(always)]
            fn from_i32_pair(x0: i32, x1: i32) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_i32(x0);
                r.x1 = <$Fp>::from_i32(x1);
                r
            }

            /// Create an element [x0, x1] from a pair of integers
            #[inline(always)]
            fn from_u64_pair(x0: u64, x1: u64) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_u64(x0);
                r.x1 = <$Fp>::from_u64(x1);
                r
            }

            /// Create an element [x0, x1] from a pair of integers
            #[inline(always)]
            fn from_i64_pair(x0: i64, x1: i64) -> Self {
                let mut r = Self::ZERO;
                r.x0 = <$Fp>::from_i64(x0);
                r.x1 = <$Fp>::from_i64(x1);
                r
            }

            /// Set the real part of the value to a small integer value
            #[inline(always)]
            fn set_x0_small(&mut self, x: i32) {
                self.x0 = <$Fp>::from_i32(x);
            }

            /// Set the real part of the value to a small integer value
            #[inline(always)]
            fn set_x1_small(&mut self, x: i32) {
                self.x1 = <$Fp>::from_i32(x);
            }

            #[inline]
            fn is_zero(self) -> u32 {
                self.x0.is_zero() & self.x1.is_zero()
            }

            #[inline]
            fn equals(&self, rhs: &Self) -> u32 {
                self.x0.equals(&rhs.x0) & self.x1.equals(&rhs.x1)
            }

            #[inline]
            fn set_add(&mut self, rhs: &Self) {
                self.x0 += &rhs.x0;
                self.x1 += &rhs.x1;
            }

            #[inline]
            fn set_sub(&mut self, rhs: &Self) {
                self.x0 -= &rhs.x0;
                self.x1 -= &rhs.x1;
            }

            #[inline]
            fn set_neg(&mut self) {
                self.x0.set_neg();
                self.x1.set_neg();
            }

            #[inline]
            fn set_conjugate(&mut self) {
                self.x1.set_neg();
            }

            #[inline]
            fn conjugate(&self) -> Self {
                Self {
                    x0: self.x0,
                    x1: -&self.x1,
                }
            }

            #[inline]
            // NOTE: old multiplication has been replaced in favour of Longa's
            // algorithm which efficiently computes sums and differences of
            // products. For more information, see the `sum_of_products()`
            // function in `fp_gen.rs`.
            fn set_mul_old(&mut self, rhs: &Self) {
                // a <- x0*y0
                // b <- x1*y1
                // c <- (x0 + x1)*(y0 + y1)
                // (x0 + i*x1)*(y0 + i*y1) = (x0*y0 - x1*y1) + i*(x0*y1 + y0*x1)
                //                         = (a - b) + i*(c - a - b)
                let a = &self.x0 * &rhs.x0;
                let b = &self.x1 * &rhs.x1;
                let c = &(&self.x0 + &self.x1) * &(&rhs.x0 + &rhs.x1);
                self.x0 = a;
                self.x0 -= &b;
                self.x1 = c;
                self.x1 -= &a;
                self.x1 -= &b;
            }

            #[inline]
            fn mul_old(self, rhs: Self) -> Self {
                let mut r = self;
                r.set_mul_old(&rhs);
                r
            }

            #[inline(always)]
            fn set_mul(&mut self, rhs: &Self) {
                // Computes x*y from:
                // x = (x0 + i*x1)
                // y = (y0 + i*y1)
                // x*y = (x0 + i*x1)*(y0 + i*y1)
                //     = (x0*y0 - x1*y1) + i*(x0*y1 + y0*x1)
                // Computes (x0*y0 - x1*y1)
                let x0 = <$Fp>::difference_of_products(&self.x0, &rhs.x0, &self.x1, &rhs.x1);
                // Computes (x0*y1 + y0*x1)
                let x1 = <$Fp>::sum_of_products(&self.x0, &rhs.x1, &self.x1, &rhs.x0);

                self.x0 = x0;
                self.x1 = x1;
            }

            #[inline]
            fn mul_new(self, rhs: Self) -> Self {
                let mut r = self;
                r.set_mul(&rhs);
                r
            }

            #[inline]
            fn set_square(&mut self) {
                // (x0 + i*x1)^2 = (x0^2 - x1^2) + 2*i*(x0*x1)
                //               = (x0 + x1)*(x0 - x1) + i*(2*x0*x1)
                let a = &self.x0 + &self.x1;
                let b = &self.x0 - &self.x1;
                self.x1 *= &self.x0;
                self.x1.set_mul2();
                self.x0 = a;
                self.x0 *= &b;
            }

            #[inline]
            fn square(self) -> Self {
                let mut r = self;
                r.set_square();
                r
            }

            #[inline]
            fn set_half(&mut self) {
                self.x0.set_half();
                self.x1.set_half();
            }

            #[inline]
            fn half(self) -> Self {
                let mut r = self;
                r.set_half();
                r
            }

            #[inline]
            fn set_mul2(&mut self) {
                self.x0.set_mul2();
                self.x1.set_mul2();
            }

            #[inline]
            fn mul2(self) -> Self {
                let mut r = self;
                r.set_mul2();
                r
            }

            #[inline]
            fn set_mul3(&mut self) {
                self.x0.set_mul3();
                self.x1.set_mul3();
            }

            #[inline]
            fn mul3(self) -> Self {
                let mut r = self;
                r.set_mul3();
                r
            }

            #[inline]
            fn set_mul4(&mut self) {
                self.x0.set_mul4();
                self.x1.set_mul4();
            }

            #[inline]
            fn mul4(self) -> Self {
                let mut r = self;
                r.set_mul4();
                r
            }

            #[inline]
            fn set_mul8(&mut self) {
                self.x0.set_mul8();
                self.x1.set_mul8();
            }

            #[inline]
            fn mul8(self) -> Self {
                let mut r = self;
                r.set_mul8();
                r
            }

            #[inline]
            fn set_mul_small(&mut self, k: i32) {
                self.x0.set_mul_small(k);
                self.x1.set_mul_small(k);
            }

            #[inline]
            fn mul_small(self, k: i32) -> Self {
                let mut r = self;
                r.set_mul_small(k);
                r
            }

            #[inline]
            fn set_select(&mut self, a: &Self, b: &Self, ctl: u32) {
                self.x0.set_select(&a.x0, &b.x0, ctl);
                self.x1.set_select(&a.x1, &b.x1, ctl);
            }

            #[inline]
            fn select(a: &Self, b: &Self, ctl: u32) -> Self {
                Self {
                    x0: <$Fp>::select(&a.x0, &b.x0, ctl),
                    x1: <$Fp>::select(&a.x1, &b.x1, ctl),
                }
            }

            #[inline]
            fn set_cond(&mut self, rhs: &Self, ctl: u32) {
                self.x0.set_cond(&rhs.x0, ctl);
                self.x1.set_cond(&rhs.x1, ctl);
            }

            #[inline]
            fn set_condneg(&mut self, ctl: u32) {
                let y0 = -(&self.x0);
                let y1 = -(&self.x1);
                self.x0.set_cond(&y0, ctl);
                self.x1.set_cond(&y1, ctl);
            }

            #[inline]
            fn condswap(a: &mut Self, b: &mut Self, ctl: u32) {
                <$Fp>::condswap(&mut a.x0, &mut b.x0, ctl);
                <$Fp>::condswap(&mut a.x1, &mut b.x1, ctl);
            }

            #[inline]
            fn set_div(&mut self, rhs: &Self) {
                // 1/(x0 + i*x1) = (x0 - i*x1)/(x0^2 + x1^2)
                let mut z = rhs.x0.square();
                z += &rhs.x1.square();
                z.set_invert();
                let mut r = *rhs;
                r.x1.set_neg();
                r.x0 *= &z;
                r.x1 *= &z;
                self.set_mul(&r);
            }

            #[inline]
            fn set_invert(&mut self) {
                // 1/(x0 + i*x1) = (x0 - i*x1)/(x0^2 + x1^2)
                let mut z = self.x0.square();
                z += &self.x1.square();
                z.set_invert();
                self.x0 *= &z;
                self.x1 *= &z;
                self.x1.set_neg();
            }

            #[inline]
            fn invert(self) -> Self {
                let mut r = self;
                r.set_invert();
                r
            }

            /// Legendre symbol on this value. Return value is:
            ///   0   if this value is zero
            ///  +1   if this value is a non-zero quadratic residue
            ///  -1   if this value is not a quadratic residue
            #[inline]
            fn legendre(self) -> i32 {
                // x = x0 + i*x1 is a square in GF(p^2) if and only if
                // x0^2 + x1^2 is a square in GF(p). Moreover, x0^2 + x1^2 is
                // zero if and only if x is zero.
                (self.x0.square() + self.x1.square()).legendre()
            }

            /// Return `0xFFFFFFFF` when this value is a square in GF(p^2) and
            /// `0x00000000` otherwise.
            #[inline]
            fn is_square(self) -> u32 {
                !((self.legendre() >> 1) as u32)
            }

            /// Return `0xFFFFFFFF` when this value is a square in GF(p) and
            /// `0x00000000` otherwise.
            #[inline]
            fn is_square_base_field(self) -> u32 {
                // x = x0 + i*x1 is a square in GF(p) if and only if
                // x0 is a square and x1 is zero;
                self.x0.is_square() & self.x1.is_zero()
            }

            /// Set this value to its square root. Returned value is 0xFFFFFFFF if
            /// the operation succeeded (value was indeed a quadratic residue), or
            /// 0x00000000 otherwise. On success, the chosen root is the one whose
            /// sign is 0 (i.e. if the "real part" is non-zero, then it is an even
            /// integer; if the "real part" is zero, then the "imaginary part" is
            /// an even integer). On failure, this value is set to 0.
            fn set_sqrt(&mut self) -> u32 {
                // x^p = (x0 + i*x1)^p = x0 - i*x1  (Frobenius automorphism)
                // Thus: x^(p+1) = (x0 + i*x1)*(x0 - i*x1) = x0^2 + x1^2, which
                // is an element of GF(p). All elements of GF(p) are squares in
                // GF(p^2), but x0^2 + x1^2 is not necessarily a square in GF(p).
                //
                // Let conj(p) = x^p = x0 - i*x1. Note that conj() is analogous to
                // the conjugate in complex numbers. In particular:
                //    conj(a + b) = conj(a) + conj(b)
                //    conj(a * b) = conj(a) * conj(b)
                // This implies that conj(x) is a square if and only if x is a
                // square, and conj(sqrt(x)) = sqrt(conj(x)). Thus, if x is a
                // square, then:
                //    (sqrt(x)*conj(sqrt(x)))^2 = x*conj(x) = x0^2 + x1^2
                // But sqrt(x)*conj(sqrt(x)) is in GF(p); therefore, if x is a
                // square, then x0^2 + x1^2 must be a square in GF(p).
                //
                // Suppose that y = y0 + i*y1 such that y^2 = x. Then:
                //   y0^2 - y1^2 = x0
                //   2*y0*y1 = x1
                // If x1 = 0 then:
                //    if x0.legendre() >= 0 then y = sqrt(x0)
                //                          else y = i*sqrt(-x0)
                // else:
                //    y0 != 0 (necessarily) and y1 = x1 / (2*y0)
                //    Thus:
                //       y0^4 - x0*y0^2 - (x1^2)/4 = 0
                //    Discriminant is delta = x0^2 + x1^2, which is always a square
                //    (see above). Therefore:
                //       y0^2 = (x0 +/- sqrt(delta))/2
                //    We can thus compute (x0 + sqrt(delta))/2 and check its
                //    Legendre symbol; we subtract sqrt(delta) from it if it is
                //    not a square. We then extract y0 as a square root of the
                //    result, and compute y1 from it.
                //
                // Main cost is the two square roots in GF(p) (for delta and
                // for y0); Legendre symbols and inversions are vastly faster.

                // sqrt_delta <- sqrt(x0^2 + x1^2)
                let (sqrt_delta, r1) = (self.x0.square() + self.x1.square()).sqrt();
                // y0sq <- (x0 + sqrt(delta)) / 2
                let mut y0sq = (self.x0 + sqrt_delta).half();
                // If x1 = 0, then replace y0sq with x0
                let x1z = self.x1.is_zero();
                y0sq.set_cond(&self.x0, x1z);
                // Get the Legendre symbol and set nqr to 0xFFFFFFFF when y0sq
                // is not a square
                let ls = y0sq.legendre();
                let nqr = (ls >> 1) as u32;
                // If not a square:
                //    if x1 = 0, then y0sq contains x0 and we want -x0
                //    if x1 != 0, then y0sq <- y0sq - sqrt(delta)
                y0sq.set_condneg(nqr & x1z);
                y0sq.set_cond(&(y0sq - sqrt_delta), nqr & !x1z);
                // Get the square root.
                let (mut y0, r2) = y0sq.sqrt();
                let r = r1 & r2;
                // Compute y1 = x1 / (2*y0).
                let mut y1 = self.x1 / y0.mul2();
                // If x1 = 0, then the square root worked, and y1 = 0 at this point;
                // we must still exchange y0 and y1 if x0 was not a square.
                <$Fp>::condswap(&mut y0, &mut y1, nqr & x1z);
                // Result goes into this object. If there was a failure (r == 0),
                // then we must clear both x0 and x1.
                self.x0.set_select(&<$Fp>::ZERO, &y0, r);
                self.x1.set_select(&<$Fp>::ZERO, &y1, r);
                // Sign mangement: negate the result if needed.
                let x0odd = ((self.x0.encode()[0] as u32) & 1).wrapping_neg();
                let x1odd = ((self.x1.encode()[0] as u32) & 1).wrapping_neg();
                let x0z = self.x0.is_zero();
                self.set_condneg(x0odd | (x0z & x1odd));
                r
            }

            fn sqrt(self) -> (Self, u32) {
                let mut y = self;
                let r = y.set_sqrt();
                (y, r)
            }

            /// Set this value to its fourth root. Returned value is 0xFFFFFFFF if
            /// the operation succeeded (value was indeed a fourth root), or
            /// 0x00000000 otherwise. On success, the chosen root is the one whose
            /// sign is 0 (i.e. if the "real part" is non-zero, then it is an even
            /// integer; if the "real part" is zero, then the "imaginary part" is
            /// an even integer). On failure, this value is set to 0.
            fn set_fourth_root(&mut self) -> u32 {
                // The aim of this function is to generalise set_sqrt by finding
                // an element of Fp^2, y = y0 + i*y1 such that x = x0 + i x1 = y^4
                //
                // Ultimately, this is done by writing out relationships between
                // xi and yi to solve a quadratic equation.
                //
                // If we have y^4 = (y0 + i*y1)^4 = x0 + i*x1 then:
                //     x0 = y0^4 - 6*y0^2*y1^2 + y1^4
                //     x1 = 4*y0*y1*(y0^2 - y1^2)
                // Additionally, using that the norm is multiplicative,
                // we have that
                //
                //     norm(x) = (x0^2 + x1^2)
                //     norm(y) = n = (y0^2 + y1^2) = norm(x)^4
                //
                // We can compute n = y0^2 + y1^2 with only a fourth-root
                // in Fp: n = (x0^2 + x1^2)^((p+1) / 8)
                // where we use p = 7 mod 8, otherwise we do two sqrt which
                // will be slower (2x the cost).
                //
                // Combining the expanded result and the norm equation
                // gives a quartic polynomial in y0 which only appears
                // with even powers:
                //
                //    8*y0^4 - 8*n*y0^2 + n^2 - x0 = 0
                //    y0^4 - n*y0^2 + (n^2 - x0) / 8 = 0
                //
                // We can write this as a quadratic equation in y0^2
                // and solve for y0^2 as:
                //
                //    y0^2 = (n ± sqrt(n^2 - (n^2 - x0)/2)) / 2
                //
                // and so y0^2 is recovered from the sqrt of the disc.
                //    disc = n^2 - (n^2 - x0)/2 = (n^2 + x0)/2
                //
                // To recover y0 itself we require one last sqrt in Fp
                // which one of the two values
                //
                //    y0^2 = sqrt(n ± sqrt_disc) / 2
                //
                // We can tell which value to pick by looking at the
                // legendre symbol of y0^2 and flipping the sign of n
                // when a QNR is found.
                //
                // Finally, we can compute y1 from the above using that
                //     y1 = x1 / (4 * y0 * sqrt_disc)
                //
                // TODO: explain edge cases carefully.
                let norm = self.x0.square() + self.x1.square();
                let (mut n, r1) = norm.fourth_root();
                // Now we need to solve a quadratic equation for y0
                // 8y0^4 - 8ny0^2 + n^2 - x0 = 0
                // The disc of this polynomial is given as
                // y0^2 = [8n + sqrt(32(n^2 + x0))] / 16
                // disc = 32(n^2 + x0)
                let disc = (n.square() + self.x0).half();

                // This has a solution, so we can always take a sqrt
                let (disc_sqrt, r2) = disc.sqrt();

                // Solving this polynomial gives y0^2, the solution
                // will be one of these two, which we pick by ensuring
                // y0^2 has a rational sqrt
                let mut y02 = (disc_sqrt + n).half();

                // Computing y0 means taking a sqrt. First, we
                // need to check if the sqrt is rational in Fp
                // If y0^2 is not a square, we use y0^2 - n
                // and also flip the sign of n
                let lsy02 = y02.legendre();
                let nqr = (lsy02 >> 1) as u32;
                y02.set_cond(&(y02 - n), nqr);
                n.set_condneg(nqr);

                // When y0^2 is zero, the correct value
                // is insead n, so we can do a conditional
                // swap
                y02.set_cond(&n, y02.is_zero());

                // Now we can take the sqrt no problem, for all
                // cases!
                let (y0, r3) = y02.sqrt();

                // y1 is computed from y0 with an inversion for
                // all cases, except when x1 = 0 (see below)
                let mut y1 = self.x1 / (y0 * disc_sqrt.mul4());

                // The final check comes from the case when x1 = 0
                // Generally, we have that:
                //     If x1 == 0 and x0 is a square, y1 = 0
                //     If x1 == 0 and x0 is not a square, y1 = y0
                //
                // However, when x1 == 0 then y1 is already zero, so
                // all we need to account for is the case when we need
                // to set y1 = y0.
                //
                // if x1 is zero and x0 is a NQR then we want to return
                // F(y0, y0) so we conditionally set y1 = y0
                // Rather than check whether x0 is a square, we can instead
                // check whether the discrim. is zero in this case
                y1.set_cond(&y0, self.x1.is_zero() & disc.is_zero());

                // As long has nothing bad has happened, we can
                // now return the fourth root. If any of the r are
                // falsey, we return 0
                let r = r1 & r2 & r3;
                self.x0.set_select(&<$Fp>::ZERO, &y0, r);
                self.x1.set_select(&<$Fp>::ZERO, &y1, r);

                // Sign mangement: negate the result if needed.
                let x0odd = ((self.x0.encode()[0] as u32) & 1).wrapping_neg();
                let x1odd = ((self.x1.encode()[0] as u32) & 1).wrapping_neg();
                let x0z = self.x0.is_zero();
                self.set_condneg(x0odd | (x0z & x1odd));

                return r;
            }

            fn fourth_root(self) -> (Self, u32) {
                let mut y = self;
                let r = y.set_fourth_root();
                (y, r)
            }

            /// Raise this value to the power e. Exponent e is encoded in
            /// unsigned little-endian convention over exactly ebitlen bits.
            fn set_pow(&mut self, e: &[u8], ebitlen: usize) {
                self.set_pow_ext(e, 0, ebitlen);
            }

            /// Raise this value to the power e. Exponent e is encoded in
            /// unsigned little-endian convention, over exactly ebitlen bits,
            /// and starting at the bit offset eoff.
            fn set_pow_ext(&mut self, e: &[u8], eoff: usize, ebitlen: usize) {
                // TODO: implement a window optimization to make fewer
                // multiplications.
                let x = *self;
                *self = Self::ONE;
                for i in (eoff..(eoff + ebitlen)).rev() {
                    let y = &*self * &x;
                    let ctl = (((e[i >> 3] >> (i & 7)) as u32) & 1).wrapping_neg();
                    self.set_cond(&y, ctl);
                    if i == eoff {
                        break;
                    }
                    self.set_square();
                }
            }

            /// Return this value to the power e (as a new element). Exponent e
            /// is encoded in unsigned little-endian convention over exactly
            /// ebitlen bits.
            fn pow(self, e: &[u8], ebitlen: usize) -> Self {
                let mut x = self;
                x.set_pow(e, ebitlen);
                x
            }

            /// Return this value to the power e (as a new element). Exponent e
            /// is encoded in unsigned little-endian convention over exactly
            /// ebitlen bits, and starting at the bit offset eoff.
            fn pow_ext(self, e: &[u8], eoff: usize, ebitlen: usize) -> Self {
                let mut x = self;
                x.set_pow_ext(e, eoff, ebitlen);
                x
            }

            fn encode(self) -> [u8; Self::ENCODED_LENGTH] {
                let mut r = [0u8; Self::ENCODED_LENGTH];
                r[..<$Fp>::ENCODED_LENGTH].copy_from_slice(&self.x0.encode());
                r[<$Fp>::ENCODED_LENGTH..].copy_from_slice(&self.x1.encode());
                r
            }

            fn decode(buf: &[u8]) -> (Self, u32) {
                if buf.len() != Self::ENCODED_LENGTH {
                    return (Self::ZERO, 0);
                }
                let (mut x0, c0) = <$Fp>::decode(&buf[..<$Fp>::ENCODED_LENGTH]);
                let (mut x1, c1) = <$Fp>::decode(&buf[<$Fp>::ENCODED_LENGTH..]);
                let cx = c0 & c1;
                x0.set_cond(&<$Fp>::ZERO, !cx);
                x1.set_cond(&<$Fp>::ZERO, !cx);
                (Self { x0, x1 }, cx)
            }

            /// Decode the provided bytes into a field element. The source slice
            /// can have arbitrary length; the bytes are interpreted with the
            /// unsigned little-endian convention (no sign bit), with the first half
            /// of the bytes corresponding to x0 and the latter half to x1. For each
            /// resulting integer, the result is reduced modulo the field modulus p.
            /// By definition, this function does not enforce canonicality of the source
            /// value.
            fn decode_reduce(buf: &[u8]) -> Self {
                let n = buf.len() >> 1;
                let x0 = <$Fp>::decode_reduce(&buf[..n]);
                let x1 = <$Fp>::decode_reduce(&buf[n..]);
                Self { x0, x1 }
            }

            /// Set this structure to a random field element (indistinguishable
            /// from uniform generation).
            fn set_rand<T: ::rand_core::CryptoRng + ::rand_core::RngCore>(&mut self, rng: &mut T) {
                self.x0.set_rand(rng);
                self.x1.set_rand(rng);
            }

            /// Return a new random field element (indistinguishable from
            /// uniform generation).
            fn rand<T: ::rand_core::CryptoRng + ::rand_core::RngCore>(rng: &mut T) -> Self {
                let mut x = Self::ZERO;
                x.set_rand(rng);
                x
            }

            /// Raise this value to the power e. The exponent length (in bits)
            /// MUST be at most ebitlen. This is constant-time for both the
            /// base value (self) and the exponent (e); the exponent maximum
            /// size (ebitlen) is considered non-secret.
            fn set_pow_u64(&mut self, e: u64, ebitlen: usize) {
                match ebitlen {
                    0 => {
                        *self = Self::ONE;
                    }
                    1 => {
                        self.set_cond(&Self::ONE, ((e as u32) & 1).wrapping_sub(1));
                    }
                    _ => {
                        let x = *self;
                        self.set_cond(
                            &Self::ONE,
                            (((e >> (ebitlen - 1)) as u32) & 1).wrapping_sub(1),
                        );
                        for i in (0..(ebitlen - 1)).rev() {
                            self.set_square();
                            let y = &*self * &x;
                            self.set_cond(&y, (((e >> i) as u32) & 1).wrapping_neg());
                        }
                    }
                }
            }

            /// Return this value to the power e. The exponent length (in bits)
            /// MUST be at most ebitlen. This is constant-time for both the
            /// base value (self) and the exponent (e); the exponent maximum
            /// size (ebitlen) is considered non-secret.
            fn pow_u64(self, e: u64, ebitlen: usize) -> Self {
                let mut x = self;
                x.set_pow_u64(e, ebitlen);
                x
            }

            /// Raise this value to the power e. The exponent is considered
            /// non-secret.
            fn set_pow_u64_vartime(&mut self, e: u64) {
                match e {
                    0 => {
                        *self = Self::ONE;
                    }
                    1 => {
                        return;
                    }
                    2 => {
                        self.set_square();
                    }
                    3 => {
                        *self *= self.square();
                    }
                    4 => {
                        self.set_square();
                        self.set_square();
                    }
                    _ => {
                        let xx = self.square();
                        let xw = [*self, xx, xx * &*self];
                        let mut j = 63 - e.leading_zeros();
                        j &= !1u32;
                        *self = xw[((e >> j) as usize) - 1];
                        while j > 0 {
                            j -= 2;
                            self.set_square();
                            self.set_square();
                            let k = ((e >> j) as usize) & 3;
                            if k > 0 {
                                self.set_mul(&xw[k - 1]);
                            }
                        }
                    }
                }
            }

            /// Return this value to the power e. The exponent is considered
            /// non-secret.
            fn pow_u64_vartime(self, e: u64) -> Self {
                let mut x = self;
                x.set_pow_u64_vartime(e);
                x
            }

            /// Get the "hash" of the value. For x = x0 + i*x1, this is:
            ///    (hashcode(x0) << 1) | (hashcode(x1) & 1)
            /// i.e. bit 0 is bit 0 of x1, and bits 1..63 are bits 0..62 of x0
            /// (both in Montgomery representation).
            fn hashcode(self) -> u64 {
                (self.x0.hashcode() << 1) | (self.x1.hashcode() & 1)
            }

            fn batch_invert(xx: &mut [Self]) {
                // We use Montgomery's trick:
                //   1/u = v*(1/(u*v))
                //   1/v = u*(1/(u*v))
                // Applied recursively on n elements, this computes an inversion
                // with a single inversion in the field, and 3*(n-1) multiplications.
                // We use batches of 200 elements; larger batches only yield
                // moderate improvements, while sticking to a fixed moderate batch
                // size allows stack-based allocation.
                let n = xx.len();
                let mut i = 0;
                while i < n {
                    let blen = if (n - i) > 200 { 200 } else { n - i };
                    let mut tt = [Self::ZERO; 200];
                    tt[0] = xx[i];
                    let zz0 = tt[0].is_zero();
                    tt[0].set_cond(&Self::ONE, zz0);
                    for j in 1..blen {
                        tt[j] = xx[i + j];
                        tt[j].set_cond(&Self::ONE, tt[j].is_zero());
                        tt[j] *= tt[j - 1];
                    }
                    let mut k = Self::ONE / tt[blen - 1];
                    for j in (1..blen).rev() {
                        let mut x = xx[i + j];
                        let zz = x.is_zero();
                        x.set_cond(&Self::ONE, zz);
                        xx[i + j].set_cond(&(k * tt[j - 1]), !zz);
                        k *= x;
                    }
                    xx[i].set_cond(&k, !zz0);
                    i += blen;
                }
            }

            /// Precompute an array of indicies to optimally compute the look-up table for
            /// discrete log computations for elements of order 2^n.
            fn precompute_dlp_table_index(n: usize) -> Vec<usize> {
                // TODO: this may not be the fastest method when this table is pre-computed on
                // the fly, and this should be experimented with in the future.
                fn dlp_table_index_inner(dd: &mut Vec<usize>, base: usize, n: usize) {
                    dd.push(base);
                    if n == 1 {
                        return;
                    }
                    let n0 = n >> 1;
                    let n1 = n - n0;
                    dlp_table_index_inner(dd, base + n1, n0);
                    dlp_table_index_inner(dd, base + n0, n1);
                }

                let mut dd = Vec::new();
                dlp_table_index_inner(&mut dd, 0, n);
                dd.sort();

                let mut dd2 = Vec::new();
                for i in 0..dd.len() {
                    let v = dd[i];
                    let j = dd2.len();
                    if j == 0 || v != dd2[j - 1] {
                        dd2.push(v);
                    }
                }
                dd2
            }

            /// Precompute two vectors of values used to optimally solve the dlog
            /// for elements of order 2^n exactly.
            ///
            /// Explicitly, this involves computing:
            /// - A table dlog_table of indicies corresponding to where to split
            ///   the dlog recursively of type Vec<usize>
            /// - A table of Fp2 elements `gpp[j] = g^(2^dlog_table[j])` of type
            ///   of type Vec<Self>
            ///
            /// Note that the first value (gpp[0]) is g itself, and the last one must
            /// be -1 (otherwise, g does not have order exactly 2^e).
            fn precompute_dlp_tables(self, n: usize) -> (Vec<usize>, Vec<Self>, u32) {
                // First compute a table of indicies, we will compute and store
                // the values g^(2^dlog_table[j])
                let dlog_table = Self::precompute_dlp_table_index(n);
                let mut gpp = vec![Self::ZERO; dlog_table.len()];

                // Compute g^(2^dlog_table[j])
                gpp[0] = self;
                let mut j = 1;
                let mut g = self;
                let mut lg = 0;
                while j < gpp.len() {
                    g.set_square();
                    lg += 1;
                    if lg == dlog_table[j] {
                        gpp[j] = g;
                        j += 1;
                    }
                }

                // Ensure that that g has indeed order exactly n.
                let ok = g.equals(&Self::MINUS_ONE);

                (dlog_table, gpp, ok)
            }

            /// Inner function for solving DLP with order 2^e.
            /// If gk = -1, then base is self; otherwise, it is gpp[gk]
            /// (equal to self^(2^dlog_table[gk])). Order of the base is 2^lg.
            /// Output (of size lg bits) is written in v[], starting at offset
            /// voff (counted in bits). The target bit values MUST be all zero
            /// initially in v[] (non-target bits are not modified).
            /// Returned value is `0xFFFFFFFF` on success, `x00000000` on error.
            #[allow(clippy::too_many_arguments)]
            fn solve_dlp_n_inner(
                self,
                gpp: &Vec<Self>,
                gk: usize,
                x: &Self,
                v: &mut [u8],
                voff: usize,
                e: usize,
                dlog_table: &Vec<usize>,
            ) -> u32 {
                let lg = e - dlog_table[gk];

                // At the deepest recursion level, lg = 1, g = -1,
                // and x = 1 or -1.
                if lg == 1 {
                    let hz = x.x1.is_zero();
                    let lp = x.x0.equals(&<$Fp>::ONE);
                    let ln = x.x0.equals(&<$Fp>::MINUS_ONE);
                    v[voff >> 3] |= ((ln & 1) << (voff & 7)) as u8;
                    return hz & (lp | ln);
                }

                // Split lg = lg0 + lg1.
                // Precomputed indices (in dlog_table) assume that the split is
                // done such that lg0 = floor(lg/2).
                let lg0 = lg >> 1;
                let lg1 = lg - lg0;

                // Solve for v0.
                //   g' = g^(2^lg1)
                //   x' = x^(2^lg1)
                let mut gk0 = gk + 1;
                while dlog_table[gk0] != e - lg0 {
                    gk0 += 1;
                }
                let mut x0 = *x;
                for _ in 0..lg1 {
                    x0.set_square();
                }
                let ok0 = self.solve_dlp_n_inner(gpp, gk0, &x0, v, voff, e, dlog_table);

                // Solve for v1.
                //   g' = g^(2^lg0)
                //   x' = x/g^v0
                let mut gk1 = gk + 1;
                while dlog_table[gk1] != e - lg1 {
                    gk1 += 1;
                }
                let mut x1 = gpp[gk].conjugate();
                x1.set_pow_ext(v, voff, lg0);
                x1 *= x;
                let ok1 = self.solve_dlp_n_inner(gpp, gk1, &x1, v, voff + lg0, e, dlog_table);

                ok0 & ok1
            }

            /// Find integer v (modulo 2^e) such that x = self^v. If self
            /// has order exactly 2^e, and there is a solution v, then this
            /// function returns (v, 0xFFFFFFFF). If self does not have order
            /// exactly 2^e (including if self^(2^(e-1)) = 1, i.e. the order of
            /// self is a strict divisor or 2^e), or if there is no solution,
            /// then this function returns (0, 0).
            ///
            /// Optionally include precomputed values from the method precompute_dlp_tables
            /// otherwise these are computed at runtime.
            fn solve_dlp_2e(
                self,
                x: &Self,
                e: usize,
                precomputed_tables: Option<(&Vec<usize>, &Vec<Self>)>,
            ) -> (Vec<u8>, u32) {
                // Method: consider g, x and lg such that:
                //   g has multiplicative order 2^lg
                //   x = g^v for some v (in the 0 to 2^lg-1 range)
                // If lg = 1 then g = -1, and x = 0 or -1.
                //   -> if g != -1, or x is not 0 or -1, then the input is
                //      erroneous and we can report it
                // If lg > 1:
                //   Let lg0 = floor(lg / 2) and lg1 = lg - lg0.
                //   Let v = v0 + (2^lg0)*v1
                //   Then:
                //      x^(2^lg1) = (g^(2^lg1))^v0
                //   We get v0 with a recursive call on base g^(2^lg1) and
                //   value x^(2^lg1). Once we have v0:
                //      x/g^v0 = (g^(2^lg0))^v1
                //   Another recursive call on base g^(2^lg0) and value
                //   x/g^v0 yields v1, from which we easily obtain v.
                //   Note that 1/g = conj(g), since g is a 2n-th root of 1.
                //
                // We avoid recomputing the same g^(2^lg) values by keeping
                // the relevant values in a local array; the important indices
                // are the ones specified in the dlog_table array.

                // Use the function precompute_dlp_table to precompute the
                // values g^(2^lg), keeping the relevant values
                // in the gpp[] array. We have:
                //    gpp[j] = g^(2^dlog_table[j])
                // Note that the first value (gpp[0]) is g itself, and the
                // last one must be -1 (otherwise, g does not have order
                // exactly n).
                //

                // If a user has supplied the precomputations, unpack them, otherwise compute
                // them at runtime.
                // TODO: I don't like the clone here, but I don't see a way around borrow lifetimes...
                let (dlog_table, gpp, ok0) = match precomputed_tables {
                    Some((exps, values)) => (exps.clone(), values.clone(), u32::MAX),
                    None => self.precompute_dlp_tables(e),
                };

                // Apply the recursion.
                let mut v = vec![0u8; (e + 7) >> 3];
                let ok1 = self.solve_dlp_n_inner(&gpp, 0, x, &mut v, 0, e, &dlog_table);
                (v, ok0 & ok1)
            }

            /// Decode an element from bytes, no check is made that the input
            /// value is reduced except that the buffer is of the excpected
            /// length of `Self::ENCODED_LENGTH` (handled within the Fp decode).
            pub const fn const_decode_no_check(x0_buf: &[u8], x1_buf: &[u8]) -> Self {
                let x0 = <$Fp>::const_decode_no_check(&x0_buf);
                let x1 = <$Fp>::const_decode_no_check(&x1_buf);
                Self { x0, x1 }
            }
        }

        // ========================================================================

        impl ::std::fmt::Display for $typename {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "i*{} + {}", self.x1, self.x0)
            }
        }

        impl ::core::ops::Add<$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn add(self, other: $typename) -> $typename {
                let mut r = self;
                r.set_add(&other);
                r
            }
        }

        impl ::core::ops::Add<&$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn add(self, other: &$typename) -> $typename {
                let mut r = self;
                r.set_add(other);
                r
            }
        }

        impl ::core::ops::Add<$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn add(self, other: $typename) -> $typename {
                let mut r = *self;
                r.set_add(&other);
                r
            }
        }

        impl ::core::ops::Add<&$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn add(self, other: &$typename) -> $typename {
                let mut r = *self;
                r.set_add(other);
                r
            }
        }

        impl ::core::ops::AddAssign<$typename> for $typename {
            #[inline(always)]
            fn add_assign(&mut self, other: $typename) {
                self.set_add(&other);
            }
        }

        impl ::core::ops::AddAssign<&$typename> for $typename {
            #[inline(always)]
            fn add_assign(&mut self, other: &$typename) {
                self.set_add(other);
            }
        }

        impl ::core::ops::Div<$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn div(self, other: $typename) -> $typename {
                let mut r = self;
                r.set_div(&other);
                r
            }
        }

        impl ::core::ops::Div<&$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn div(self, other: &$typename) -> $typename {
                let mut r = self;
                r.set_div(other);
                r
            }
        }

        impl ::core::ops::Div<$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn div(self, other: $typename) -> $typename {
                let mut r = *self;
                r.set_div(&other);
                r
            }
        }

        impl ::core::ops::Div<&$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn div(self, other: &$typename) -> $typename {
                let mut r = *self;
                r.set_div(other);
                r
            }
        }

        impl ::core::ops::DivAssign<$typename> for $typename {
            #[inline(always)]
            fn div_assign(&mut self, other: $typename) {
                self.set_div(&other);
            }
        }

        impl ::core::ops::DivAssign<&$typename> for $typename {
            #[inline(always)]
            fn div_assign(&mut self, other: &$typename) {
                self.set_div(other);
            }
        }

        impl ::core::ops::Mul<$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn mul(self, other: $typename) -> $typename {
                let mut r = self;
                r.set_mul(&other);
                r
            }
        }

        impl ::core::ops::Mul<&$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn mul(self, other: &$typename) -> $typename {
                let mut r = self;
                r.set_mul(other);
                r
            }
        }

        impl ::core::ops::Mul<$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn mul(self, other: $typename) -> $typename {
                let mut r = *self;
                r.set_mul(&other);
                r
            }
        }

        impl ::core::ops::Mul<&$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn mul(self, other: &$typename) -> $typename {
                let mut r = *self;
                r.set_mul(other);
                r
            }
        }

        impl ::core::ops::MulAssign<$typename> for $typename {
            #[inline(always)]
            fn mul_assign(&mut self, other: $typename) {
                self.set_mul(&other);
            }
        }

        impl ::core::ops::MulAssign<&$typename> for $typename {
            #[inline(always)]
            fn mul_assign(&mut self, other: &$typename) {
                self.set_mul(other);
            }
        }

        impl ::core::ops::Neg for $typename {
            type Output = $typename;

            #[inline(always)]
            fn neg(self) -> $typename {
                let mut r = self;
                r.set_neg();
                r
            }
        }

        impl ::core::ops::Neg for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn neg(self) -> $typename {
                let mut r = *self;
                r.set_neg();
                r
            }
        }

        impl ::core::ops::Sub<$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn sub(self, other: $typename) -> $typename {
                let mut r = self;
                r.set_sub(&other);
                r
            }
        }

        impl ::core::ops::Sub<&$typename> for $typename {
            type Output = $typename;

            #[inline(always)]
            fn sub(self, other: &$typename) -> $typename {
                let mut r = self;
                r.set_sub(other);
                r
            }
        }

        impl ::core::ops::Sub<$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn sub(self, other: $typename) -> $typename {
                let mut r = *self;
                r.set_sub(&other);
                r
            }
        }

        impl ::core::ops::Sub<&$typename> for &$typename {
            type Output = $typename;

            #[inline(always)]
            fn sub(self, other: &$typename) -> $typename {
                let mut r = *self;
                r.set_sub(other);
                r
            }
        }

        impl ::core::ops::SubAssign<$typename> for $typename {
            #[inline(always)]
            fn sub_assign(&mut self, other: $typename) {
                self.set_sub(&other);
            }
        }

        impl ::core::ops::SubAssign<&$typename> for $typename {
            #[inline(always)]
            fn sub_assign(&mut self, other: &$typename) {
                self.set_sub(other);
            }
        }

        impl $crate::fq::Fq for $typename {
            // Reexport constants for Trait
            const ENCODED_LENGTH: usize = Self::ENCODED_LENGTH;
            const ZERO: Self = Self::ZERO;
            const ONE: Self = Self::ONE;
            const TWO: Self = Self::TWO;
            const THREE: Self = Self::THREE;
            const FOUR: Self = Self::FOUR;
            const MINUS_ONE: Self = Self::MINUS_ONE;
            const ZETA: Self = Self::ZETA;
            const MINUS_ZETA: Self = Self::ZETA;

            // Re-export functions for Trait
            // Technically, we could define many of the above functions
            // directly here rather than in `impl $typename`, but it felt
            // easier to read to keep everything together above and re-export
            // for the trait in a consistant way.
            //
            // Rust friends:
            // I am very happy to have feedback on ways to refactor this!

            /// Return the value x + i*0 for a given integer x of type `i32`.
            fn from_i32(x: i32) -> Self {
                Self::from_i32(x)
            }

            /// Return the value x + i*0 for a given integer x of type `u32`.
            fn from_u32(x: u32) -> Self {
                Self::from_u32(x)
            }

            /// Return the value x + i*0 for a given integer x of type `i64`.
            fn from_i64(x: i64) -> Self {
                Self::from_i64(x)
            }

            /// Return the value x + i*0 for a given integer x of type `u64`.
            fn from_u64(x: u64) -> Self {
                Self::from_u64(x)
            }

            /// Return the value x0 + i*x1 for a given two integers of type `i32`.
            fn from_i32_pair(x0: i32, x1: i32) -> Self {
                Self::from_i32_pair(x0, x1)
            }

            /// Return the value x0 + i*x1 for a given two integers of type `u32`.
            fn from_u32_pair(x0: u32, x1: u32) -> Self {
                Self::from_u32_pair(x0, x1)
            }

            /// Return the value x0 + i*x1 for a given two integers of type `i64`.
            fn from_i64_pair(x0: i64, x1: i64) -> Self {
                Self::from_i64_pair(x0, x1)
            }

            /// Return the value x0 + i*x1 for a given two integers of type `u64`.
            fn from_u64_pair(x0: u64, x1: u64) -> Self {
                Self::from_u64_pair(x0, x1)
            }

            /// Set the "real" component of self to an integer of type `i32` in place.
            fn set_x0_small(&mut self, x: i32) {
                self.set_x0_small(x)
            }

            /// Set the "imaginary" component of self to an integer of type `i32` in place.
            fn set_x1_small(&mut self, x: i32) {
                self.set_x1_small(x)
            }

            fn is_zero(self) -> u32 {
                self.is_zero()
            }
            fn equals(self, rhs: &Self) -> u32 {
                self.equals(rhs)
            }

            fn set_neg(&mut self) {
                self.set_neg()
            }
            fn set_half(&mut self) {
                self.set_half()
            }
            fn set_mul2(&mut self) {
                self.set_mul2()
            }
            fn set_mul3(&mut self) {
                self.set_mul3()
            }
            fn set_mul4(&mut self) {
                self.set_mul4()
            }
            fn set_mul8(&mut self) {
                self.set_mul8()
            }

            fn half(self) -> Self {
                self.half()
            }
            fn mul2(self) -> Self {
                self.mul2()
            }
            fn mul3(self) -> Self {
                self.mul3()
            }
            fn mul4(self) -> Self {
                self.mul4()
            }
            fn mul8(self) -> Self {
                self.mul8()
            }

            fn set_conjugate(&mut self) {
                self.set_conjugate();
            }
            fn set_mul_small(&mut self, k: i32) {
                self.set_mul_small(k)
            }
            fn set_square(&mut self) {
                self.set_square()
            }
            fn set_invert(&mut self) {
                self.set_invert()
            }
            fn set_pow(&mut self, e: &[u8], ebitlen: usize) {
                self.set_pow(e, ebitlen)
            }
            fn set_pow_ext(&mut self, e: &[u8], eoff: usize, ebitlen: usize) {
                self.set_pow_ext(e, eoff, ebitlen)
            }
            fn set_pow_u64(&mut self, e: u64, ebitlen: usize) {
                self.set_pow_u64(e, ebitlen)
            }
            fn set_pow_u64_vartime(&mut self, e: u64) {
                self.set_pow_u64_vartime(e)
            }

            fn conjugate(self) -> Self {
                self.conjugate()
            }
            fn mul_small(self, k: i32) -> Self {
                self.mul_small(k)
            }
            fn square(self) -> Self {
                self.square()
            }
            fn invert(self) -> Self {
                self.invert()
            }
            fn pow(self, e: &[u8], ebitlen: usize) -> Self {
                self.pow(e, ebitlen)
            }
            fn pow_ext(self, e: &[u8], eoff: usize, ebitlen: usize) -> Self {
                self.pow_ext(e, eoff, ebitlen)
            }
            fn pow_u64(&mut self, e: u64, ebitlen: usize) -> Self {
                self.pow_u64(e, ebitlen)
            }
            fn pow_u64_vartime(&mut self, e: u64) -> Self {
                self.pow_u64_vartime(e)
            }

            fn set_sqrt(&mut self) -> u32 {
                self.set_sqrt()
            }
            fn set_fourth_root(&mut self) -> u32 {
                self.set_fourth_root()
            }
            fn sqrt(self) -> (Self, u32) {
                self.sqrt()
            }
            fn fourth_root(self) -> (Self, u32) {
                self.fourth_root()
            }

            fn legendre(self) -> i32 {
                self.legendre()
            }

            fn is_square(self) -> u32 {
                self.is_square()
            }

            fn is_square_base_field(self) -> u32 {
                self.is_square_base_field()
            }

            fn batch_invert(xx: &mut [Self]) {
                Self::batch_invert(xx)
            }

            fn precompute_dlp_tables(self, n: usize) -> (Vec<usize>, Vec<Self>, u32) {
                self.precompute_dlp_tables(n)
            }

            fn solve_dlp_2e(
                self,
                x: &Self,
                e: usize,
                precomputed_tables: Option<(&Vec<usize>, &Vec<Self>)>,
            ) -> (Vec<u8>, u32) {
                self.solve_dlp_2e(x, e, precomputed_tables)
            }

            fn set_select(&mut self, a: &Self, b: &Self, ctl: u32) {
                self.set_select(a, b, ctl)
            }
            fn set_cond(&mut self, rhs: &Self, ctl: u32) {
                self.set_cond(rhs, ctl)
            }
            fn set_condneg(&mut self, ctl: u32) {
                self.set_condneg(ctl)
            }
            fn select(a: &Self, b: &Self, ctl: u32) -> Self {
                Self::select(a, b, ctl)
            }
            fn condswap(a: &mut Self, b: &mut Self, ctl: u32) {
                Self::condswap(a, b, ctl)
            }

            fn encode(self) -> [u8; Self::ENCODED_LENGTH] {
                self.encode()
            }
            fn decode(buf: &[u8]) -> (Self, u32) {
                Self::decode(buf)
            }
            fn decode_reduce(buf: &[u8]) -> Self {
                Self::decode_reduce(buf)
            }

            fn set_rand<R: ::rand_core::CryptoRng + ::rand_core::RngCore>(&mut self, rng: &mut R) {
                self.set_rand(rng)
            }
            fn rand<R: ::rand_core::CryptoRng + ::rand_core::RngCore>(rng: &mut R) -> Self {
                Self::rand(rng)
            }

            fn hashcode(self) -> u64 {
                self.hashcode()
            }
        }
    };
} // End of macro: define_fp2_from_type

/// A macro to define the degree two extension of the finite field Fp, with
/// modulus x^2 + 1 directly from the modulus.
/// All functions are designed to run in constant time.
///
/// Macro expectations:
/// - A typename for the finite field Fp^2 generated.
/// - A typename for the base finite field Fp.
/// - An array of `N` words which represent the finite field characteristic
///   in base 2^64
#[macro_export]
macro_rules! define_fp2_from_modulus {
    (
        typename = $typename:ident,
        base_typename = $base_typename:ident,
        modulus = $modulus:expr,
    ) => {
        $crate::define_fp_core!(typename = $base_typename, modulus = $modulus,);
        $crate::define_fp2_from_type!(typename = $typename, base_field = $base_typename,);
    };
} // End of macro: define_fp2_from_modulus
