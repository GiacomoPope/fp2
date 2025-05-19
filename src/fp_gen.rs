//! A macro to define efficient and constant-time arithmetic for finite fields Fp with
//! p = 3 mod 4 using Montgomery multiplication.
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

/// A macro to define the finite field Fp, all functions are designed to run in
/// constant time. Assumes that the characteristic is p = 3 mod 4.
///
/// Macro expectations:
/// - A typename for the finite field generated
/// - An array of `N` words which represent the finite field characteristic
///   in base 2^64
#[macro_export]
macro_rules! define_fp_core {
    (
        typename = $typename:ident,
        modulus = $modulus:expr,
    ) => {
        /// A finite field element. Contents are opaque.
        /// All functions are constant-time.
        ///
        /// A field element x is encoded into bytes by using the unsigned
        /// little-endian convention over the unique representant of x in the
        /// [0..(p-1)] range. There is no sign bit.
        #[derive(Clone, Copy, Debug)]
        pub struct $typename([u64; $typename::N]);

        impl $typename {
            // IMPLEMENTATION NOTES
            // --------------------
            //
            // Modulus is p. Each element is represented over N limbs, in base
            // 2^64.
            //
            // Let R = 2^(64*N) mod p. A field element x is represented by
            // the integer x*R mod p, in the [0..(p-1)] range. The limbs are in
            // little-endian order (limb 0 is the least significant).
            //
            // Multiplications use Montgomery multiplication: given x and y,
            // the value (x*y)/R mod p is computed. Since our values are in
            // Montgomery representation, what is computed is really
            // (x*R)*(y*R)/R = x*y*R mod p, which is the correct product. The
            // decoding and encoding functions apply the required conversions; the
            // use of Montgomery representation is not visible to other code using
            // this type.

            // Number of words and bit length of the field characteristic
            pub const N: usize = Self::top_word_index() + 1;
            pub const BIT_LENGTH: usize = Self::mod_bitlen();
            pub const MODULUS: [u64; Self::N] = $modulus;

            // Multiplier for decode_reduce().
            const CLEN: usize = 8 * (Self::N - 1);
            const TDEC: Self = Self::pow2mod((2 * Self::N - 1) * 64);

            // Constants used for internal arithmetic
            const P0I: u64 = Self::ninv64(Self::MODULUS[0]);
            const R: Self = Self::pow2mod(Self::N * 64);
            const R2: Self = Self::pow2mod(Self::N * 128);
            const P1: u64 = Self::top_u32();
            const P1DIV_M: u64 =
                1 + ((((((1u64 << 32) - Self::P1) as u128) << 64) / (Self::P1 as u128)) as u64);
            const NUM1: usize = (2 * Self::BIT_LENGTH - 34) / 31;
            const NUM2: usize = 2 * Self::BIT_LENGTH - 31 * Self::NUM1 - 2;
            const TFIXDIV: Self = Self::const_mmul(
                Self::const_mmul(Self::pow2mod(Self::NUM1 * 33 + 64 - Self::NUM2), Self::R2),
                Self::R2,
            );
            const SQRT_EXP: [u64; Self::N] = Self::const_sqrt_exp();
            const FOURTH_ROOT_EXP: [u64; Self::N] = Self::const_fourth_root_exp();

            // Predefined constants used externally
            pub const ZERO: Self = Self([0u64; Self::N]);
            pub const ONE: Self = Self::R;
            pub const TWO: Self = Self::const_small(2);
            pub const THREE: Self = Self::const_small(3);
            pub const FOUR: Self = Self::const_small(4);
            pub const MINUS_ONE: Self = Self::const_neg(Self::R);

            /// Encoding length of a field element (in bytes). All elements
            /// always encode into exactly that many bytes. Encoding is
            /// canonical: a given field element has a unique valid encoding,
            /// and the decoding process verifies that this specific encoding
            /// was used.
            pub const ENCODED_LENGTH: usize = (Self::BIT_LENGTH + 7) >> 3;

            pub const fn new(input: [u64; Self::N]) -> Self {
                return Self(input);
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            pub fn from_i32(x: i32) -> Self {
                // For the i32 type, we can avoid the mul by R2 in from_u64
                // by instead using the cheaper set_mul_small with the ONE constant.
                let mut r = Self::ONE;
                r.set_mul_small(x);
                r
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            pub fn from_i64(x: i64) -> Self {
                let sx = (x >> 63) as u64;
                let ax = ((x as u64) ^ sx).wrapping_sub(sx);
                let mut r = Self::from_u64(ax);
                r.set_condneg(sx as u32);
                r
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            pub fn from_u32(x: u32) -> Self {
                Self::from_u64(x as u64)
            }

            /// Create an element by converting the provided integer.
            #[inline(always)]
            pub fn from_u64(x: u64) -> Self {
                let mut r = Self::ZERO;
                r.0[0] = x;
                r.set_mul(&Self::R2);
                r
            }

            /// Return 0xFFFFFFFF if this value is zero, or 0x00000000 otherwise.
            #[inline]
            pub fn is_zero(self) -> u32 {
                let mut x = self.0[0];
                for i in 1..Self::N {
                    x |= self.0[i];
                }
                (!$crate::utils64::sgnw(x | x.wrapping_neg())) as u32
            }

            /// Return 0xFFFFFFFF if this value is equal to rhs, or 0x00000000
            /// otherwise.
            #[inline(always)]
            pub fn equals(self, rhs: &Self) -> u32 {
                let mut r = 0u64;
                for i in 0..Self::N {
                    r |= self.0[i] ^ rhs.0[i];
                }
                (((r | r.wrapping_neg()) >> 63) as u32).wrapping_sub(1)
            }

            /// Add `rhs` to this value.
            #[inline]
            fn set_add(&mut self, rhs: &Self) {
                // raw addition.
                let mut cc1 = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::addcarry_u64(self.0[i], rhs.0[i], cc1);
                    self.0[i] = d;
                    cc1 = ee;
                }

                // subtract modulus.
                let mut cc2 = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::subborrow_u64(self.0[i], Self::MODULUS[i], cc2);
                    self.0[i] = d;
                    cc2 = ee;
                }

                // add back modulus if the result was negative, i.e. cc1 - cc2 < 0.
                let mm = (cc1 as u64).wrapping_sub(cc2 as u64);
                let mut cc = 0;
                for i in 0..Self::N {
                    let (d, ee) =
                        $crate::utils64::addcarry_u64(self.0[i], mm & Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }
            }

            /// Subtract `rhs` from this value.
            #[inline]
            fn set_sub(&mut self, rhs: &Self) {
                // raw subtraction
                let mut cc = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::subborrow_u64(self.0[i], rhs.0[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }

                // add back modulus if the result was negative
                let mm = (cc as u64).wrapping_neg();
                cc = 0;
                for i in 0..Self::N {
                    let (d, ee) =
                        $crate::utils64::addcarry_u64(self.0[i], mm & Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }
            }

            /// Negate this value.
            #[inline]
            pub fn set_neg(&mut self) {
                // subtract from zero
                let mut cc = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::subborrow_u64(0, self.0[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }

                // add back the modulus if needed (i.e. if input was non-zero)
                let mm = (cc as u64).wrapping_neg();
                cc = 0;
                for i in 0..Self::N {
                    let (d, ee) =
                        $crate::utils64::addcarry_u64(self.0[i], mm & Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }
            }

            // Perform Montgomery reduction (division by R) on this value.
            // Internal note: if self has proper contents (value less than p), then
            // this necessarily yields a properly reduced value. If self is not
            // properly reduced, then the output is in [0..p] inclusive.
            #[inline]
            fn set_montyred(&mut self) {
                for _ in 0..Self::N {
                    let f = self.0[0].wrapping_mul(Self::P0I);
                    let (_, mut cc) = $crate::utils64::umull_add(f, Self::MODULUS[0], self.0[0]);
                    for i in 1..Self::N {
                        let (d, hi) =
                            $crate::utils64::umull_add2(f, Self::MODULUS[i], self.0[i], cc);
                        self.0[i - 1] = d;
                        cc = hi;
                    }
                    self.0[Self::N - 1] = cc;
                }
            }

            /// Multiply this value by `rhs`.
            #[inline]
            fn set_mul(&mut self, rhs: &Self) {
                let mut t = Self::ZERO;

                // combined muls + reduction
                let mut cch = 0;
                for i in 0..Self::N {
                    let f = rhs.0[i];
                    let (lo, mut cc1) = $crate::utils64::umull_add(f, self.0[0], t.0[0]);
                    let g = lo.wrapping_mul(Self::P0I);
                    let (_, mut cc2) = $crate::utils64::umull_add(g, Self::MODULUS[0], lo);
                    for j in 1..Self::N {
                        let (d, hi1) = $crate::utils64::umull_add2(f, self.0[j], t.0[j], cc1);
                        cc1 = hi1;
                        let (d, hi2) = $crate::utils64::umull_add2(g, Self::MODULUS[j], d, cc2);
                        cc2 = hi2;
                        t.0[j - 1] = d;
                    }
                    let (d, ee) = $crate::utils64::addcarry_u64(cc1, cc2, cch);
                    t.0[Self::N - 1] = d;
                    cch = ee;
                }

                // final reduction: subtract modulus if necessary
                let mut cc = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::subborrow_u64(t.0[i], Self::MODULUS[i], cc);
                    t.0[i] = d;
                    cc = ee;
                }
                let mm = (cch as u64).wrapping_sub(cc as u64);
                cc = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::addcarry_u64(t.0[i], mm & Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }
            }

            /// Replace this value with its square.
            #[inline]
            pub fn set_square(&mut self) {
                // FIXME: this turns out to be slower than set_mul() on x86_64
                // when N >= 23. This is probably due to the more complicated
                // loop bounds. Full unrolling helps, but can only be done at
                // the crate level:
                //   RUSTFLAGS="-C llvm-args=-unroll-threshold=1200"
                // This impacts all the code in the crate, and is thus
                // probably not a very good idea.

                // Compute the square over integers.
                let mut t = [0u64; Self::N << 1];

                // sum_{i<j} a_i*a_j*2^(64*(i+j)) < 2^(64*(2*N-1))
                // -> t[2*N-1] remains at zero
                let f = self.0[0];
                let (d, mut cc) = $crate::utils64::umull(f, self.0[1]);
                t[1] = d;
                for j in 2..Self::N {
                    let (d, hi) = $crate::utils64::umull_add(f, self.0[j], cc);
                    t[j] = d;
                    cc = hi;
                }
                t[Self::N] = cc;
                for i in 1..(Self::N - 1) {
                    let f = self.0[i];
                    let (d, mut cc) = $crate::utils64::umull_add(f, self.0[i + 1], t[(i << 1) + 1]);
                    t[(i << 1) + 1] = d;
                    for j in (i + 2)..Self::N {
                        let (d, hi) = $crate::utils64::umull_add2(f, self.0[j], t[i + j], cc);
                        t[i + j] = d;
                        cc = hi;
                    }
                    t[i + Self::N] = cc;
                }

                // Double the partial sum.
                // -> t contains sum_{i!=j} a_i*a_j*2^(64*(i+j))
                let mut cc = 0;
                for i in 1..((Self::N << 1) - 1) {
                    let w = t[i];
                    let ee = w >> 63;
                    t[i] = (w << 1) | cc;
                    cc = ee;
                }
                t[(Self::N << 1) - 1] = cc;

                // Add the squares a_i*a_i*w^(64*2*i).
                let mut cc = 0;
                for i in 0..Self::N {
                    let (lo, hi) = $crate::utils64::umull(self.0[i], self.0[i]);
                    let (d0, ee) = $crate::utils64::addcarry_u64(lo, t[i << 1], cc);
                    let (d1, ee) = $crate::utils64::addcarry_u64(hi, t[(i << 1) + 1], ee);
                    t[i << 1] = d0;
                    t[(i << 1) + 1] = d1;
                    cc = ee;
                }

                // Apply Montgomery reduction. We use the following facts:
                //  - upper half is necessarily less than p
                //  - set_montyred() accepts a full-limbs input and outputs a
                //    value of at most p
                //  - set_add() tolerates an input operand equal to p provided
                //    that the sum is less than 2*p
                self.0.copy_from_slice(&t[..Self::N]);
                self.set_montyred();
                let mut y = Self([0u64; Self::N]);
                y.0.copy_from_slice(&t[Self::N..]);
                self.set_add(&y);
            }

            /// Compute the square of this value.
            #[inline(always)]
            pub fn square(self) -> Self {
                let mut r = self;
                r.set_square();
                r
            }

            /// Square this value n times in place
            #[inline(always)]
            pub fn set_xsquare(&mut self, n: u32) {
                for _ in 0..n {
                    self.set_square();
                }
            }

            /// Square this value n times
            #[inline(always)]
            pub fn xsquare(self, n: u32) -> Self {
                let mut r = self;
                r.set_xsquare(n);
                r
            }

            /// Halve this value.
            #[inline]
            pub fn set_half(&mut self) {
                let m = (self.0[0] & 1).wrapping_neg();
                let (mut dd, mut cc) =
                    $crate::utils64::addcarry_u64(self.0[0], m & Self::MODULUS[0], 0);
                dd >>= 1;
                for i in 1..Self::N {
                    let (x, ee) =
                        $crate::utils64::addcarry_u64(self.0[i], m & Self::MODULUS[i], cc);
                    cc = ee;
                    self.0[i - 1] = dd | (x << 63);
                    dd = x >> 1;
                }
                self.0[Self::N - 1] = dd | ((cc as u64) << 63);
            }

            /// Compute the half of this value.
            #[inline(always)]
            pub fn half(self) -> Self {
                let mut r = self;
                r.set_half();
                r
            }

            /// Double this value.
            #[inline]
            pub fn set_mul2(&mut self) {
                // Double (as an integer) and subtract the modulus.
                let mut cc = 0;
                let mut tb = 0;
                for i in 0..Self::N {
                    let w = self.0[i];
                    let t = (w << 1) | tb;
                    tb = w >> 63;
                    let (d, ee) = $crate::utils64::subborrow_u64(t, Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }

                // add back modulus if the result was negative
                let mm = tb.wrapping_sub(cc as u64);
                cc = 0;
                for i in 0..Self::N {
                    let (d, ee) =
                        $crate::utils64::addcarry_u64(self.0[i], mm & Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }
            }

            /// Compute the sum of this value with itself.
            #[inline(always)]
            pub fn mul2(self) -> Self {
                let mut r = self;
                r.set_mul2();
                r
            }

            /// Triple this value.
            #[inline]
            pub fn set_mul3(&mut self) {
                let r = self.mul2();
                *self += &r;
            }

            /// Compute the triple of this value.
            #[inline(always)]
            pub fn mul3(self) -> Self {
                let mut r = self;
                r.set_mul3();
                r
            }

            /// Quadruple this value.
            #[inline]
            pub fn set_mul4(&mut self) {
                self.set_mul2();
                self.set_mul2();
            }

            /// Compute the quadruple of this value.
            #[inline(always)]
            pub fn mul4(self) -> Self {
                let mut r = self;
                r.set_mul4();
                r
            }

            /// Multiply this value by 8
            #[inline]
            pub fn set_mul8(&mut self) {
                self.set_mul2();
                self.set_mul2();
                self.set_mul2();
            }

            /// Compute 8 times this value.
            #[inline(always)]
            pub fn mul8(self) -> Self {
                let mut r = self;
                r.set_mul8();
                r
            }

            /// Multiply this value by a small signed integer k.
            #[inline]
            pub fn set_mul_small(&mut self, k: i32) {
                // Get the absolute value of the multiplier (but remember the sign).
                let sk = (k >> 31) as u32;
                let ak = ((k as u32) ^ sk).wrapping_sub(sk);

                // Do the product over integers.
                let (d, mut hi) = $crate::utils64::umull(self.0[0], ak as u64);
                self.0[0] = d;
                for i in 1..Self::N {
                    let (d, ee) = $crate::utils64::umull_add(self.0[i], ak as u64, hi);
                    self.0[i] = d;
                    hi = ee;
                }

                // We write:
                //    p = p1*2^m + p0   (modulus)
                //    x = x1*2^m + x0   (unreduced product)
                // with:
                //    2^31 <= p1 < 2^32
                //    0 <= p0 < 2^m
                //    0 <= x0 < 2^m
                // Since the current value x is the product of the input (less
                // than p) by a multiplier of at most 2^31, we know that:
                //    0 <= x < p*2^31 < 2^(63+m)
                //    0 <= x1 < 2^63.
                // We compute:
                //    b = floor(x1/p1)
                // Analysis shows that floor(x/p) = b, b-1 or b+1.
                //
                // We thus obtain b, then increment it (unless b == p1), then
                // subtract b*p from x; we then add back p repeatedly until a
                // non-negative result is obtained. At most two conditional
                // additions are needed to achieve that result.
                //
                // Division by p1 can be done with the Granlund-Montgomery method:
                //    https://dl.acm.org/doi/10.1145/773473.178249
                // (LLVM usually applies that method, but may fail to do so if for
                // instance optimizing for code size on some platforms, thus it is
                // best to apply the method explicitly so that constant-time code
                // is more reliably achieved.)

                // Extract top word of x.
                let bl = Self::BIT_LENGTH & 63;
                let x1 = if bl == 0 {
                    (self.0[Self::N - 1] >> 32) | (hi << 32)
                } else if bl < 32 {
                    (self.0[Self::N - 1] << (32 - bl)) | (self.0[Self::N - 2] >> (32 + bl))
                } else if bl == 32 {
                    self.0[Self::N - 1]
                } else {
                    (hi << (96 - bl)) | (self.0[Self::N - 1] >> (bl - 32))
                };

                // Compute b = floor(x1/p1).
                let (_, t) = $crate::utils64::umull(x1, Self::P1DIV_M);
                let b = (x1.wrapping_sub(t) >> 1).wrapping_add(t) >> 31;

                // Add 1 to b, unless b == p1 (we cannot have b > p1).
                let b = b + (Self::P1.wrapping_sub(b) >> 63);

                // Subtract b*p from x.
                let mut cc1 = 0;
                let mut cc2 = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::umull_add(b, Self::MODULUS[i], cc1);
                    cc1 = ee;
                    let (d, ee) = $crate::utils64::subborrow_u64(self.0[i], d, cc2);
                    self.0[i] = d;
                    cc2 = ee;
                }
                let (mut hi, _) = $crate::utils64::subborrow_u64(hi, cc1, cc2);

                // Add p (at most twice) as long as the value is negative.
                for _ in 0..2 {
                    let m = $crate::utils64::sgnw(hi);
                    let mut cc = 0;
                    for i in 0..Self::N {
                        let (d, ee) =
                            $crate::utils64::addcarry_u64(self.0[i], m & Self::MODULUS[i], cc);
                        self.0[i] = d;
                        cc = ee;
                    }
                    hi = hi.wrapping_add(cc as u64);
                }

                // We computed self*|k|; we must adjust for the sign of k.
                self.set_condneg(sk);
            }

            /// Compute the product of this value by a small (unsigned) integer k.
            #[inline(always)]
            pub fn mul_small(self, k: i32) -> Self {
                let mut r = self;
                r.set_mul_small(k);
                r
            }

            /// Set this value to either a or b, depending on whether the control
            /// word ctl is 0x00000000 or 0xFFFFFFFF, respectively.
            /// The value of ctl MUST be either 0x00000000 or 0xFFFFFFFF.
            #[inline]
            pub fn set_select(&mut self, a: &Self, b: &Self, ctl: u32) {
                let c = (ctl as u64) | ((ctl as u64) << 32);
                for i in 0..Self::N {
                    let wa = a.0[i];
                    let wb = b.0[i];
                    self.0[i] = wa ^ (c & (wa ^ wb));
                }
            }

            /// Return a or b, if ctl is 0x00000000 or 0xFFFFFFFF, respectively.
            /// ctl MUST be either 0x00000000 or 0xFFFFFFFF.
            /// The value of ctl MUST be either 0x00000000 or 0xFFFFFFFF.
            #[inline]
            pub fn select(a: &Self, b: &Self, ctl: u32) -> Self {
                let mut r = Self::ZERO;
                r.set_select(a, b, ctl);
                r
            }

            /// Set this value to rhs if ctl is 0xFFFFFFFF; leave it unchanged if
            /// ctl is 0x00000000.
            /// The value of ctl MUST be either 0x00000000 or 0xFFFFFFFF.
            #[inline]
            pub fn set_cond(&mut self, rhs: &Self, ctl: u32) {
                let c = (ctl as u64) | ((ctl as u64) << 32);
                for i in 0..Self::N {
                    let wa = self.0[i];
                    let wb = rhs.0[i];
                    self.0[i] = wa ^ (c & (wa ^ wb));
                }
            }

            /// Negate this value if ctl is 0xFFFFFFFF; leave it unchanged if
            /// ctl is 0x00000000.
            /// The value of ctl MUST be either 0x00000000 or 0xFFFFFFFF.
            #[inline]
            pub fn set_condneg(&mut self, ctl: u32) {
                let v = -(self as &Self);
                self.set_cond(&v, ctl);
            }

            /// Exchange the values of a and b is ctl is 0xFFFFFFFF; leave both
            /// values unchanged if ctl is 0x00000000.
            /// The value of ctl MUST be either 0x00000000 or 0xFFFFFFFF.
            #[inline]
            pub fn condswap(a: &mut Self, b: &mut Self, ctl: u32) {
                let c = (ctl as u64) | ((ctl as u64) << 32);
                for i in 0..Self::N {
                    let wa = a.0[i];
                    let wb = b.0[i];
                    let wc = c & (wa ^ wb);
                    a.0[i] = wa ^ wc;
                    b.0[i] = wb ^ wc;
                }
            }

            // Set this value to (u*f+v*g)/2^64. Coefficients f
            // and g are provided as u64, but they are signed integers in the
            // [-2^62..+2^62] range.
            #[inline]
            fn set_montylin(&mut self, u: &Self, v: &Self, f: u64, g: u64) {
                // Make sure f and g are non-negative.
                let sf = $crate::utils64::sgnw(f);
                let f = (f ^ sf).wrapping_sub(sf);
                let tu = Self::select(u, &-u, sf as u32);
                let sg = $crate::utils64::sgnw(g);
                let g = (g ^ sg).wrapping_sub(sg);
                let tv = Self::select(v, &-v, sg as u32);

                let (d, mut cc) = $crate::utils64::umull_x2(tu.0[0], f, tv.0[0], g);
                self.0[0] = d;
                for i in 1..Self::N {
                    let (d, hi) = $crate::utils64::umull_x2_add(tu.0[i], f, tv.0[i], g, cc);
                    self.0[i] = d;
                    cc = hi;
                }
                let up = cc;

                // Montgomery reduction (one round)
                let k = self.0[0].wrapping_mul(Self::P0I);
                let (_, mut cc) = $crate::utils64::umull_add(k, Self::MODULUS[0], self.0[0]);
                for i in 1..Self::N {
                    let (d, hi) = $crate::utils64::umull_add2(k, Self::MODULUS[i], self.0[i], cc);
                    self.0[i - 1] = d;
                    cc = hi;
                }
                let (d, cc1) = $crate::utils64::addcarry_u64(up, cc, 0);
                self.0[Self::N - 1] = d;

                // |f| <= 2^62 and |g| <= 2^62, therefore |u*f + v*g| < p*2^63
                // We added less than p*2^64, and divided by 2^64, so the result
                // is less than 2*p and a single conditional subtraction is enough.
                let mut cc2 = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::subborrow_u64(self.0[i], Self::MODULUS[i], cc2);
                    self.0[i] = d;
                    cc2 = ee;
                }
                let mm = (cc1 as u64).wrapping_sub(cc2 as u64);
                let mut cc = 0;
                for i in 0..Self::N {
                    let (d, ee) =
                        $crate::utils64::addcarry_u64(self.0[i], mm & Self::MODULUS[i], cc);
                    self.0[i] = d;
                    cc = ee;
                }
            }

            #[inline(always)]
            fn montylin(a: &Self, b: &Self, f: u64, g: u64) -> Self {
                let mut r = Self::ZERO;
                r.set_montylin(a, b, f, g);
                r
            }

            // Set this value to abs((a*f + b*g)/2^31). Values a and b are
            // interpreted as plain integers (not modular). Coefficients f and
            // g are provided as u64 but they really are signed integers in the
            // [-2^31..+2^31] range (inclusive). The low 31 bits of a*f + b*g
            // are dropped (i.e. the division is assumed to be exact). The result
            // is assumed to fit in N limbs (extra high bits, if any, are
            // dropped). The absolute value of (a*f + b*g)/2^31 is computed.
            // Returned value is -1 (as a u64) if a*f + b*g was negative, 0
            // otherwise.
            #[inline]
            fn set_lindiv31abs(&mut self, a: &Self, b: &Self, f: u64, g: u64) -> u64 {
                // Replace f and g with abs(f) and abs(g), but remember the
                // original signs.
                let sf = $crate::utils64::sgnw(f);
                let f = (f ^ sf).wrapping_sub(sf);
                let sg = $crate::utils64::sgnw(g);
                let g = (g ^ sg).wrapping_sub(sg);

                // Compute a*f + b*g (upper word in 'up')
                let mut cc1 = 0;
                let mut cc2 = 0;
                let mut cc3 = 0;
                for i in 0..Self::N {
                    let (d1, ee1) = $crate::utils64::subborrow_u64(a.0[i] ^ sf, sf, cc1);
                    cc1 = ee1;
                    let (d2, ee2) = $crate::utils64::subborrow_u64(b.0[i] ^ sg, sg, cc2);
                    cc2 = ee2;
                    let (d3, hi3) = $crate::utils64::umull_x2_add(d1, f, d2, g, cc3);
                    self.0[i] = d3;
                    cc3 = hi3;
                }
                let up = cc3
                    .wrapping_sub((cc1 as u64).wrapping_neg() & f)
                    .wrapping_sub((cc2 as u64).wrapping_neg() & g);

                // Right-shift the result by 31 bits.
                for i in 0..(Self::N - 1) {
                    self.0[i] = (self.0[i] >> 31) | (self.0[i + 1] << 33);
                }
                self.0[Self::N - 1] = (self.0[Self::N - 1] >> 31) | (up << 33);

                // Negate the result if (a*f + b*g) was negative.
                let w = $crate::utils64::sgnw(up);
                let mut cc = 0;
                for i in 0..Self::N {
                    let (d, ee) = $crate::utils64::subborrow_u64(self.0[i] ^ w, w, cc);
                    self.0[i] = d;
                    cc = ee;
                }

                w
            }

            #[inline(always)]
            fn lindiv31abs(a: &Self, b: &Self, f: u64, g: u64) -> (Self, u64) {
                let mut r = Self::ZERO;
                let ng = r.set_lindiv31abs(a, b, f, g);
                (r, ng)
            }

            /// Divide this value by `y`. If `y` is zero, then this sets this value
            /// to zero.
            fn set_div(&mut self, y: &Self) {
                // a <- y
                // b <- p (modulus)
                // u <- x (self)
                // v <- 0
                //
                // Invariants:
                //    a*x = y*u mod p
                //    b*x = y*v mod p
                //    b is always odd
                //
                // At each step:
                //    if a is even, then:
                //        a <- a/2, u <- u/2 mod p
                //    else:
                //        if a < b:
                //            (a, u, b, v) <- (b, v, a, u)
                //        a <- (a - b)/2
                //        u <- (u - v)/2 mod p
                //
                // We optimize this algorithm following:
                //    https://eprint.iacr.org/2020/972

                let mut a = *y;
                let mut b = Self(Self::MODULUS);
                let mut u = *self;
                let mut v = Self::ZERO;

                // Generic loop; each iteration reduces the sum of the sizes
                // of a and b by at least 31, and that sum starts at 2*BITLEN
                // (at most). We need to run it until the sum of the two lengths
                // is at most 64.
                for _ in 0..Self::NUM1 {
                    // Get approximations of a and b over 64 bits:
                    //  - If len(a) <= 64 and len(b) <= 64, then we just
                    //    use their values (low limbs).
                    //  - Otherwise, with n = max(len(a), len(b)), we use:
                    //       (a mod 2^31) + 2^31*floor(a / 2^(n - 33))
                    //       (b mod 2^31) + 2^31*floor(b / 2^(n - 33))
                    let mut c_hi = 0xFFFFFFFFFFFFFFFFu64;
                    let mut c_lo = 0xFFFFFFFFFFFFFFFFu64;
                    let mut a_hi = 0u64;
                    let mut a_lo = 0u64;
                    let mut b_hi = 0u64;
                    let mut b_lo = 0u64;
                    for j in (0..Self::N).rev() {
                        let aw = a.0[j];
                        let bw = b.0[j];
                        a_hi ^= (a_hi ^ aw) & c_hi;
                        a_lo ^= (a_lo ^ aw) & c_lo;
                        b_hi ^= (b_hi ^ bw) & c_hi;
                        b_lo ^= (b_lo ^ bw) & c_lo;
                        c_lo = c_hi;
                        let mw = aw | bw;
                        c_hi &= ((mw | mw.wrapping_neg()) >> 63).wrapping_sub(1);
                    }

                    // If c_lo = 0, then we grabbed two words for a and b.
                    // If c_lo != 0, then c_hi = 0 (they cannot be both non-zero
                    // since that would mean that a = b = 0, but b is odd). In that
                    // case, we grabbed one word (in a_hi and b_hi) and both values
                    // fit in 64 bits.
                    let s = $crate::utils64::lzcnt(a_hi | b_hi);
                    let mut xa = (a_hi << s) | ((a_lo >> 1) >> (63 - s));
                    let mut xb = (b_hi << s) | ((b_lo >> 1) >> (63 - s));
                    xa = (xa & 0xFFFFFFFF80000000) | (a.0[0] & 0x000000007FFFFFFF);
                    xb = (xb & 0xFFFFFFFF80000000) | (b.0[0] & 0x000000007FFFFFFF);

                    // If c_lo != 0, then we should ignore the computed xa and xb,
                    // and instead use the low limbs directly.
                    xa ^= c_lo & (xa ^ a.0[0]);
                    xb ^= c_lo & (xb ^ b.0[0]);

                    // Compute the 31 inner iterations.
                    let mut fg0 = 1u64;
                    let mut fg1 = 1u64 << 32;
                    for _ in 0..31 {
                        let a_odd = (xa & 1).wrapping_neg();
                        let (_, cc) = $crate::utils64::subborrow_u64(xa, xb, 0);
                        let swap = a_odd & (cc as u64).wrapping_neg();
                        let t1 = swap & (xa ^ xb);
                        xa ^= t1;
                        xb ^= t1;
                        let t2 = swap & (fg0 ^ fg1);
                        fg0 ^= t2;
                        fg1 ^= t2;
                        xa = xa.wrapping_sub(a_odd & xb);
                        fg0 = fg0.wrapping_sub(a_odd & fg1);
                        xa >>= 1;
                        fg1 <<= 1;
                    }
                    fg0 = fg0.wrapping_add(0x7FFFFFFF7FFFFFFF);
                    fg1 = fg1.wrapping_add(0x7FFFFFFF7FFFFFFF);
                    let f0 = (fg0 & 0xFFFFFFFF).wrapping_sub(0x7FFFFFFF);
                    let g0 = (fg0 >> 32).wrapping_sub(0x7FFFFFFF);
                    let f1 = (fg1 & 0xFFFFFFFF).wrapping_sub(0x7FFFFFFF);
                    let g1 = (fg1 >> 32).wrapping_sub(0x7FFFFFFF);

                    // Propagate updates to a, b, u and v.
                    let (na, nega) = Self::lindiv31abs(&a, &b, f0, g0);
                    let (nb, negb) = Self::lindiv31abs(&a, &b, f1, g1);
                    let f0 = (f0 ^ nega).wrapping_sub(nega);
                    let g0 = (g0 ^ nega).wrapping_sub(nega);
                    let f1 = (f1 ^ negb).wrapping_sub(negb);
                    let g1 = (g1 ^ negb).wrapping_sub(negb);
                    let nu = Self::montylin(&u, &v, f0, g0);
                    let nv = Self::montylin(&u, &v, f1, g1);
                    a = na;
                    b = nb;
                    u = nu;
                    v = nv;
                }

                // If y is non-zero, then the final GCD is 1, and
                // len(a) + len(b) <= NUM2 + 2 at this point (initially,
                // len(a) + len(b) <= 2*BITLEN, and each outer iteration reduces
                // the total by at least 31). Thus, the two values fit in one word
                // and we can finish the computation that way. We only need NUM2
                // iterations to reach the point where b = 1.
                let mut xa = a.0[0];
                let mut xb = b.0[0];
                let mut f0 = 1u64;
                let mut g0 = 0u64;
                let mut f1 = 0u64;
                let mut g1 = 1u64;
                for _ in 0..Self::NUM2 {
                    let a_odd = (xa & 1).wrapping_neg();
                    let (_, cc) = $crate::utils64::subborrow_u64(xa, xb, 0);
                    let swap = a_odd & (cc as u64).wrapping_neg();
                    let t1 = swap & (xa ^ xb);
                    xa ^= t1;
                    xb ^= t1;
                    let t2 = swap & (f0 ^ f1);
                    f0 ^= t2;
                    f1 ^= t2;
                    let t3 = swap & (g0 ^ g1);
                    g0 ^= t3;
                    g1 ^= t3;
                    xa = xa.wrapping_sub(a_odd & xb);
                    f0 = f0.wrapping_sub(a_odd & f1);
                    g0 = g0.wrapping_sub(a_odd & g1);
                    xa >>= 1;
                    f1 <<= 1;
                    g1 <<= 1;
                }

                self.set_montylin(&u, &v, f1, g1);

                // If y != 0 then b = 1 at this point. If y == 0, then we
                // force the result to zero.
                let w = !y.is_zero();
                let w = ((w as u64) << 32) | (w as u64);
                for i in 0..Self::N {
                    self.0[i] &= w;
                }

                // At this point, each outer iteration injected 31 extra doublings,
                // plus NUM2 for the last loop, for a total of NUM1*31 + NUM2.
                // Each montylin() call divided by 2^64, so in total we really
                // divided the value by 2^(64*(NUM1+1) - 31*NUM1 - NUM2).
                //
                // Moreover, both divisor and dividend were in Montgomery
                // representation, so the result is not in Montgomery representation
                // (the two R factors canceled each other). We want the result
                // in Montgomery representation, i.e. multiplied by 2^(64*N).
                // Therefore, we must multiply by 2^(33*NUM1 + 64 - NUM2 + 64*N),
                // which we need in
                self.set_mul(&Self::TFIXDIV);
            }

            pub fn set_invert(&mut self) {
                let r = *self;
                *self = Self::ONE;
                self.set_div(&r);
            }

            pub fn invert(&self) -> Self {
                let mut r = Self::ONE;
                r.set_div(self);
                r
            }

            /// Set this value to its square root. Returned value is 0xFFFFFFFF if
            /// the operation succeeded (value was indeed a quadratic residue), or
            /// 0x00000000 otherwise. On success, the chosen root is the one whose
            /// least significant bit (as an integer in [0..p-1]) is zero. On
            /// failure, this value is set to 0.
            pub fn set_sqrt(&mut self) -> u32 {
                // Compute x^((p+1)/4)
                let x = *self;
                self.set_modpow_pubexp(&Self::SQRT_EXP);

                // Check whether the square of the result equals the input and zeroize
                // on failure
                let r = self.square().equals(&x);
                let rw = (r as u64) | ((r as u64) << 32);
                for i in 0..Self::N {
                    self.0[i] &= rw;
                }

                // Normalise the output so that the LSB is zero
                let ctl = ((self.encode()[0] as u32) & 1).wrapping_neg();
                self.set_condneg(ctl);

                r
            }

            /// Compute the square root of this value. If this value is indeed a
            /// quadratic residue, then this returns (x, 0xFFFFFFFF), with x being
            /// the (unique) square root of this value whose least significant bit
            /// is zero (when normalized to an integer in [0..p-1]). If this value
            /// is not a quadratic residue, then this returns (zero, 0x00000000).
            pub fn sqrt(self) -> (Self, u32) {
                let mut x = self;
                let r = x.set_sqrt();
                (x, r)
            }

            /// Set this value to its square root. Returned value is 0xFFFFFFFF if
            /// the operation succeeded (value was indeed a quadratic residue), or
            /// 0x00000000 otherwise. On success, the chosen root is the one whose
            /// least significant bit (as an integer in [0..p-1]) is zero. On
            /// failure, this value is set to 0.
            ///
            /// When p = 7 mod 8 we can compute x^((p+1)/8), but for all other cases
            /// we fall back to the 2x slower method of computing x^((p+1)/4) twice.
            pub fn set_fourth_root(&mut self) -> u32 {
                let x = *self;

                if Self::MODULUS[0] & 7 == 7 {
                    // Compute x^((p+1)/8)
                    self.set_modpow_pubexp(&Self::FOURTH_ROOT_EXP);
                } else {
                    // Fall back to the much slower, general case of two sqrt.
                    self.set_modpow_pubexp(&Self::SQRT_EXP);
                    self.set_modpow_pubexp(&Self::SQRT_EXP);
                }

                // Check whether the square of the result equals the input and zeroize
                // on failure
                let r = self.xsquare(2).equals(&x);
                let rw = (r as u64) | ((r as u64) << 32);
                for i in 0..Self::N {
                    self.0[i] &= rw;
                }

                // Normalise the output so that the LSB is zero
                let ctl = ((self.encode()[0] as u32) & 1).wrapping_neg();
                self.set_condneg(ctl);

                r
            }

            /// Compute the fourth root of this value. If this value is indeed some
            /// element to the power of four, then this returns (x, 0xFFFFFFFF), with x being
            /// the (unique) fourth root of this value whose least significant bit
            /// is zero (when normalized to an integer in [0..p-1]). If this value
            /// is not some element to the power of four, then this returns (zero, 0x00000000).
            pub fn fourth_root(self) -> (Self, u32) {
                let mut x = self;
                let r = x.set_fourth_root();
                (x, r)
            }

            // Raise this value to the provided exponent. The exponent is non-zero
            // and is public. The exponent is encoded over N 64-bit limbs.
            fn set_modpow_pubexp(&mut self, e: &[u64; Self::N]) {
                // Make a 4-bit window; win[i] contains x^(i+1)
                let mut win = [Self::ZERO; 15];
                win[0] = *self;
                for i in 1..8 {
                    let j = i * 2;
                    win[j - 1] = win[i - 1].square();
                    win[j] = win[j - 1] * win[0];
                }

                // Explore 4-bit chunks of the exponent, high to low. Skip leading
                // chunks of value 0.
                let mut z = false;
                for i in (0..Self::N).rev() {
                    let ew = e[i];
                    for j in (0..16).rev() {
                        if z {
                            self.set_xsquare(4);
                        }
                        let c = ((ew >> (j << 2)) & 0x0F) as usize;
                        if c != 0 {
                            if z {
                                self.set_mul(&win[c - 1]);
                            } else {
                                z = true;
                                *self = win[c - 1];
                            }
                        }
                    }
                }
                if !z {
                    *self = Self::ONE;
                }
            }

            /// Raise this value to the power e. Exponent e is encoded in
            /// unsigned little-endian convention over exactly ebitlen bits.
            pub fn set_pow(&mut self, e: &[u8], ebitlen: usize) {
                self.set_pow_ext(e, 0, ebitlen);
            }

            /// Raise this value to the power e. Exponent e is encoded in
            /// unsigned little-endian convention, over exactly ebitlen bits,
            /// and starting at the bit offset eoff.
            pub fn set_pow_ext(&mut self, e: &[u8], eoff: usize, ebitlen: usize) {
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
            pub fn pow(self, e: &[u8], ebitlen: usize) -> Self {
                let mut x = self;
                x.set_pow(e, ebitlen);
                x
            }

            /// Return this value to the power e (as a new element). Exponent e
            /// is encoded in unsigned little-endian convention over exactly
            /// ebitlen bits, and starting at the bit offset eoff.
            pub fn pow_ext(self, e: &[u8], eoff: usize, ebitlen: usize) -> Self {
                let mut x = self;
                x.set_pow_ext(e, eoff, ebitlen);
                x
            }

            /// Legendre symbol on this value. Return value is:
            ///   0   if this value is zero
            ///  +1   if this value is a non-zero quadratic residue
            ///  -1   if this value is not a quadratic residue
            pub fn legendre(self) -> i32 {
                // This is the same optimized binary GCD as in division, except
                // that we do not need to keep track of u and v. We can also
                // work directly on the Montgomery representation because R = 2^1184
                // is a square.
                let mut a = self;
                let mut b = Self(Self::MODULUS);
                let mut ls = 0u64;

                // Generic loop; each iteration reduces the sum of the sizes
                // of a and b by at least 31, and that sum starts at 2*BITLEN
                // (at most). We need to run it until the sum of the two lengths
                // is at most 64.
                for _ in 0..Self::NUM1 {
                    // Get approximations of a and b over 64 bits:
                    //  - If len(a) <= 64 and len(b) <= 64, then we just
                    //    use their values (low limbs).
                    //  - Otherwise, with n = max(len(a), len(b)), we use:
                    //       (a mod 2^31) + 2^31*floor(a / 2^(n - 33))
                    //       (b mod 2^31) + 2^31*floor(b / 2^(n - 33))
                    let mut c_hi = 0xFFFFFFFFFFFFFFFFu64;
                    let mut c_lo = 0xFFFFFFFFFFFFFFFFu64;
                    let mut a_hi = 0u64;
                    let mut a_lo = 0u64;
                    let mut b_hi = 0u64;
                    let mut b_lo = 0u64;
                    for j in (0..Self::N).rev() {
                        let aw = a.0[j];
                        let bw = b.0[j];
                        a_hi ^= (a_hi ^ aw) & c_hi;
                        a_lo ^= (a_lo ^ aw) & c_lo;
                        b_hi ^= (b_hi ^ bw) & c_hi;
                        b_lo ^= (b_lo ^ bw) & c_lo;
                        c_lo = c_hi;
                        let mw = aw | bw;
                        c_hi &= ((mw | mw.wrapping_neg()) >> 63).wrapping_sub(1);
                    }

                    // If c_lo = 0, then we grabbed two words for a and b.
                    // If c_lo != 0, then c_hi = 0 (they cannot be both non-zero
                    // since that would mean that a = b = 0, but b is odd). In that
                    // case, we grabbed one word (in a_hi and b_hi) and both values
                    // fit in 64 bits.
                    let s = $crate::utils64::lzcnt(a_hi | b_hi);
                    let mut xa = (a_hi << s) | ((a_lo >> 1) >> (63 - s));
                    let mut xb = (b_hi << s) | ((b_lo >> 1) >> (63 - s));
                    xa = (xa & 0xFFFFFFFF80000000) | (a.0[0] & 0x000000007FFFFFFF);
                    xb = (xb & 0xFFFFFFFF80000000) | (b.0[0] & 0x000000007FFFFFFF);

                    // If c_lo != 0, then we should ignore the computed xa and xb,
                    // and instead use the low limbs directly.
                    xa ^= c_lo & (xa ^ a.0[0]);
                    xb ^= c_lo & (xb ^ b.0[0]);

                    // First 29 inner iterations.
                    let mut fg0 = 1u64;
                    let mut fg1 = 1u64 << 32;
                    for _ in 0..29 {
                        let a_odd = (xa & 1).wrapping_neg();
                        let (_, cc) = $crate::utils64::subborrow_u64(xa, xb, 0);
                        let swap = a_odd & (cc as u64).wrapping_neg();
                        ls ^= swap & ((xa & xb) >> 1);
                        let t1 = swap & (xa ^ xb);
                        xa ^= t1;
                        xb ^= t1;
                        let t2 = swap & (fg0 ^ fg1);
                        fg0 ^= t2;
                        fg1 ^= t2;
                        xa = xa.wrapping_sub(a_odd & xb);
                        fg0 = fg0.wrapping_sub(a_odd & fg1);
                        xa >>= 1;
                        fg1 <<= 1;
                        ls ^= xb.wrapping_add(2) >> 2;
                    }

                    // Compute the updated a and b (low words only) to get enough
                    // bits for the next two iterations.
                    let fg0z = fg0.wrapping_add(0x7FFFFFFF7FFFFFFF);
                    let fg1z = fg1.wrapping_add(0x7FFFFFFF7FFFFFFF);
                    let f0 = (fg0z & 0xFFFFFFFF).wrapping_sub(0x7FFFFFFF);
                    let g0 = (fg0z >> 32).wrapping_sub(0x7FFFFFFF);
                    let f1 = (fg1z & 0xFFFFFFFF).wrapping_sub(0x7FFFFFFF);
                    let g1 = (fg1z >> 32).wrapping_sub(0x7FFFFFFF);
                    let mut a0 = a.0[0]
                        .wrapping_mul(f0)
                        .wrapping_add(b.0[0].wrapping_mul(g0))
                        >> 29;
                    let mut b0 = a.0[0]
                        .wrapping_mul(f1)
                        .wrapping_add(b.0[0].wrapping_mul(g1))
                        >> 29;
                    for _ in 0..2 {
                        let a_odd = (xa & 1).wrapping_neg();
                        let (_, cc) = $crate::utils64::subborrow_u64(xa, xb, 0);
                        let swap = a_odd & (cc as u64).wrapping_neg();
                        ls ^= swap & ((a0 & b0) >> 1);
                        let t1 = swap & (xa ^ xb);
                        xa ^= t1;
                        xb ^= t1;
                        let t2 = swap & (fg0 ^ fg1);
                        fg0 ^= t2;
                        fg1 ^= t2;
                        let t3 = swap & (a0 ^ b0);
                        a0 ^= t3;
                        b0 ^= t3;
                        xa = xa.wrapping_sub(a_odd & xb);
                        fg0 = fg0.wrapping_sub(a_odd & fg1);
                        a0 = a0.wrapping_sub(a_odd & b0);
                        xa >>= 1;
                        fg1 <<= 1;
                        a0 >>= 1;
                        ls ^= b0.wrapping_add(2) >> 2;
                    }

                    // Propagate updates to a and b.
                    fg0 = fg0.wrapping_add(0x7FFFFFFF7FFFFFFF);
                    fg1 = fg1.wrapping_add(0x7FFFFFFF7FFFFFFF);
                    let f0 = (fg0 & 0xFFFFFFFF).wrapping_sub(0x7FFFFFFF);
                    let g0 = (fg0 >> 32).wrapping_sub(0x7FFFFFFF);
                    let f1 = (fg1 & 0xFFFFFFFF).wrapping_sub(0x7FFFFFFF);
                    let g1 = (fg1 >> 32).wrapping_sub(0x7FFFFFFF);

                    // Propagate updates to a, b, u and v.
                    let (na, nega) = Self::lindiv31abs(&a, &b, f0, g0);
                    let (nb, _) = Self::lindiv31abs(&a, &b, f1, g1);
                    ls ^= nega & (nb.0[0] >> 1);
                    a = na;
                    b = nb;
                }

                // If y is non-zero, then the final GCD is 1, and
                // len(a) + len(b) <= NUM2 + 2 at this point (initially,
                // len(a) + len(b) <= 2*BITLEN, and each outer iteration reduces
                // the total by at least 31). Thus, the two values fit in one word
                // and we can finish the computation that way. We only need NUM2
                // iterations to reach the point where b = 1.
                let mut xa = a.0[0];
                let mut xb = b.0[0];
                for _ in 0..Self::NUM2 {
                    let a_odd = (xa & 1).wrapping_neg();
                    let (_, cc) = $crate::utils64::subborrow_u64(xa, xb, 0);
                    let swap = a_odd & (cc as u64).wrapping_neg();
                    ls ^= swap & ((xa & xb) >> 1);
                    let t1 = swap & (xa ^ xb);
                    xa ^= t1;
                    xb ^= t1;
                    xa = xa.wrapping_sub(a_odd & xb);
                    xa >>= 1;
                    ls ^= xb.wrapping_add(2) >> 2;
                }

                // At this point, if the source value was not zero, then the low
                // bit of ls contains the QR status (0 = square, 1 = non-square),
                // which we need to convert to the expected value (+1 or -1).
                // If y == 0, then we return 0, per the API.
                let r = 1u32.wrapping_sub(((ls as u32) & 1) << 1);
                (r & !(self.is_zero() as u32)) as i32
            }

            /// Return `0xFFFFFFFF` when this value is a square in GF(p^2) and
            /// `0x00000000` otherwise.
            #[inline]
            fn is_square(self) -> u32 {
                !((self.legendre() >> 1) as u32)
            }

            /// Encode this value into bytes. Encoding uses little-endian, has
            /// a fixed size (for a given field), and is canonical.
            #[inline(always)]
            pub fn encode(self) -> [u8; Self::ENCODED_LENGTH] {
                let mut r = self;
                r.set_montyred();
                let mut d = [0u8; Self::ENCODED_LENGTH];
                for i in 0..(Self::N - 1) {
                    d[(i * 8)..(i * 8 + 8)].copy_from_slice(&r.0[i].to_le_bytes());
                }
                d[((Self::N - 1) * 8)..].copy_from_slice(
                    &(r.0[Self::N - 1].to_le_bytes()[..Self::ENCODED_LENGTH - (Self::N - 1) * 8]),
                );
                d
            }

            #[inline]
            fn set_decode_nocheck(&mut self, buf: &[u8]) {
                for i in 0..(Self::N - 1) {
                    self.0[i] = u64::from_le_bytes(
                        *<&[u8; 8]>::try_from(&buf[(8 * i)..(8 * i + 8)]).unwrap(),
                    );
                }
                let mut w = 0u64;
                for j in 0..(Self::ENCODED_LENGTH - (Self::N - 1) * 8) {
                    w |= (buf[(Self::N - 1) * 8 + j] as u64) << (8 * j);
                }
                self.0[Self::N - 1] = w;
            }

            /// Decode the provided bytes into a field element. Returned values
            /// are the element and 0xFFFFFFFF on success, or the zero element and
            /// 0x00000000 on failure. A failure is reported if the source slice
            /// does not have exactly the canonical encoding length of a field
            /// element (Self::ENCODED_LENGTH), or if the source encodes
            /// an integer which is not in the [0..(p-1)] range.
            #[inline(always)]
            pub fn decode(buf: &[u8]) -> (Self, u32) {
                if buf.len() != Self::ENCODED_LENGTH {
                    return (Self::ZERO, 0);
                }

                // decode raw value
                let mut r = Self::ZERO;
                r.set_decode_nocheck(buf);

                // check that the source is canonical; clear if invalid
                let (_, mut cc) = $crate::utils64::subborrow_u64(r.0[0], Self::MODULUS[0], 0);
                for i in 1..Self::N {
                    let (_, ee) = $crate::utils64::subborrow_u64(r.0[i], Self::MODULUS[i], cc);
                    cc = ee;
                }
                let m = (cc as u64).wrapping_neg();
                for i in 0..Self::N {
                    r.0[i] &= m;
                }

                // convert to Montgomery representation
                r.set_mul(&Self::R2);
                (r, m as u32)
            }

            /// Set this element by decoding the provided bytes. The source slice
            /// can have arbitrary length; the bytes are interpreted with the
            /// unsigned little-endian convention (no sign bit), and the resulting
            /// integer is reduced modulo the field modulus p. By definition, this
            /// function does not enforce canonicality of the source value.
            #[inline]
            pub fn set_decode_reduce(&mut self, buf: &[u8]) {
                let mut n = buf.len();
                if n == 0 {
                    *self = Self::ZERO;
                    return;
                }

                let mut tmp = [0u8; Self::ENCODED_LENGTH];
                let mut nn = n % Self::CLEN;
                if nn == 0 {
                    nn = Self::CLEN;
                }
                n -= nn;
                tmp[..nn].copy_from_slice(&buf[n..]);
                self.set_decode_nocheck(&tmp);

                while n > 0 {
                    n -= Self::CLEN;
                    tmp[..Self::CLEN].copy_from_slice(&buf[n..(n + Self::CLEN)]);
                    let mut d = Self::ZERO;
                    d.set_decode_nocheck(&tmp);
                    self.set_mul(&Self::TDEC);
                    self.set_add(&d);
                }

                self.set_mul(&Self::R2);
            }

            /// Decode the provided bytes into a field element. The source slice
            /// can have arbitrary length; the bytes are interpreted with the
            /// unsigned little-endian convention (no sign bit), and the resulting
            /// integer is reduced modulo the field modulus p. By definition, this
            /// function does not enforce canonicality of the source value.
            #[inline(always)]
            pub fn decode_reduce(buf: &[u8]) -> Self {
                let mut x = Self::ZERO;
                x.set_decode_reduce(buf);
                x
            }

            /// Set this structure to a random field element (indistinguishable
            /// from uniform generation).
            pub fn set_rand<T: ::rand_core::CryptoRng + ::rand_core::RngCore>(
                &mut self,
                rng: &mut T,
            ) {
                let mut tmp = [0u8; Self::ENCODED_LENGTH + 16];
                rng.fill_bytes(&mut tmp);
                self.set_decode_reduce(&tmp);
            }

            /// Return a new random field element (indistinguishable from
            /// uniform generation).
            pub fn rand<T: ::rand_core::CryptoRng + ::rand_core::RngCore>(rng: &mut T) -> Self {
                let mut x = Self::ZERO;
                x.set_rand(rng);
                x
            }

            /// Get the "hash" of the value (low 64 bits of the Montgomery
            /// representation).
            pub fn hashcode(self) -> u64 {
                self.0[0]
            }

            /// Implements Algorithm 2 from Patrick Longa's
            /// [ePrint 2022-367](https://eprint.iacr.org/2022/367) §3.
            /// Computes a1 * b1 + a2 * b2 using an optimised method intended
            /// for use in Fp2 multiplications
            #[inline(always)]
            pub fn sum_of_products(a1: &Self, b1: &Self, a2: &Self, b2: &Self) -> Self {
                // We return the Fp element u, initalised to zero
                let mut u = Self::ZERO;

                // Initalise some additional values to track carries.
                let mut carry_limb: u64 = 0u64;
                let mut hi_limb: u64 = 0u64;
                let mut hi_carry: u8 = 0u8;

                // Montgomery multiplcation is interleaved, so we loop through every limb
                // multiplying by the bottom word of on element through every word of the
                // other and then perform a single Montgomery reducton
                for j in 0..Self::N {
                    // Compute u = u + a1j * b1
                    let a1j = a1.0[j];
                    (u.0[0], carry_limb) = $crate::utils64::umull_add(a1j, b1.0[0], u.0[0]);
                    for k in 1..Self::N {
                        (u.0[k], carry_limb) =
                            $crate::utils64::umull_add2(a1j, b1.0[k], u.0[k], carry_limb);
                    }
                    (hi_limb, hi_carry) =
                        $crate::utils64::addcarry_u64(hi_limb, carry_limb, hi_carry);

                    // Compute u = u + a2j * b2
                    let a2j = a2.0[j];
                    (u.0[0], carry_limb) = $crate::utils64::umull_add(a2j, b2.0[0], u.0[0]);
                    for k in 1..Self::N {
                        (u.0[k], carry_limb) =
                            $crate::utils64::umull_add2(a2j, b2.0[k], u.0[k], carry_limb);
                    }
                    (hi_limb, hi_carry) =
                        $crate::utils64::addcarry_u64(hi_limb, carry_limb, hi_carry);

                    // Perform a single step of the Montgomery reduction process by computing
                    // q = u0 * (-1/p) mod 2^64
                    // Then computing
                    // u = (u + q * p) / 2^64
                    let q = u.0[0].wrapping_mul(Self::P0I);
                    (_, carry_limb) = $crate::utils64::umull_add(q, Self::MODULUS[0], u.0[0]);
                    for k in 1..Self::N {
                        (u.0[k - 1], carry_limb) =
                            $crate::utils64::umull_add2(q, Self::MODULUS[k], u.0[k], carry_limb);
                    }
                    (u.0[Self::N - 1], hi_carry) =
                        $crate::utils64::addcarry_u64(hi_limb, carry_limb, hi_carry);

                    // Shift the high carry into the high limb for the next iteration.
                    hi_limb = (hi_carry as u64);
                    hi_carry = 0;
                }

                // The result of the above is a value within [0, 2p)
                // We subtract the modulus to obtain something in the range [-p, 0) and
                // conditionally add the modulus back if the previous result was negative
                let mut borrow = 0;
                for i in 0..Self::N {
                    (u.0[i], borrow) =
                        $crate::utils64::subborrow_u64(u.0[i], Self::MODULUS[i], borrow);
                }
                let mask = (hi_limb).wrapping_sub(borrow as u64);
                borrow = 0;
                for i in 0..Self::N {
                    (u.0[i], borrow) =
                        $crate::utils64::addcarry_u64(u.0[i], mask & Self::MODULUS[i], borrow);
                }

                u
            }

            /// Implements Algorithm 2 from Patrick Longa's
            /// [ePrint 2022-367](https://eprint.iacr.org/2022/367) §3.
            /// Computes a1 * b1 - a2 * b2 using an optimised method intended
            /// for use in Fp2 multiplications
            #[inline(always)]
            pub fn difference_of_products(a1: &Self, b1: &Self, a2: &Self, b2: &Self) -> Self {
                // Compute p - b2
                let mut borrow = 0;
                let mut b2_minus = *b2;
                for i in 0..Self::N {
                    (b2_minus.0[i], borrow) =
                        $crate::utils64::subborrow_u64(Self::MODULUS[i], b2_minus.0[i], borrow);
                }
                // Regular sum of products (could the above be optimised into the main loop?)
                Self::sum_of_products(a1, b1, a2, &b2_minus)
            }

            /*
             * Support functions which compute constants at compile time to
             * generate macro constants, this allows a user to create the field
             * with only the modulus as input.
             *
             * These have been adapted from https://github.com/pornin/crrl/src/backend/w64/gfgen.rs
             */
            // Return -1/x mod 2^64. It is assumed that x is odd.
            const fn ninv64(x: u64) -> u64 {
                let y = 2u64.wrapping_sub(x);
                let y = y.wrapping_mul(2u64.wrapping_sub(y.wrapping_mul(x)));
                let y = y.wrapping_mul(2u64.wrapping_sub(y.wrapping_mul(x)));
                let y = y.wrapping_mul(2u64.wrapping_sub(y.wrapping_mul(x)));
                let y = y.wrapping_mul(2u64.wrapping_sub(y.wrapping_mul(x)));
                let y = y.wrapping_mul(2u64.wrapping_sub(y.wrapping_mul(x)));
                y.wrapping_neg()
            }

            // Custom add-with-carry, for use in const (compile-time) contexts.
            const fn adc(x: u64, y: u64, cc: u64) -> (u64, u64) {
                let z = (x as u128) + (y as u128) + (cc as u128);
                (z as u64, (z >> 64) as u64)
            }

            // Custom sub-with-borrow, for use in const (compile-time) contexts.
            const fn sbb(x: u64, y: u64, cc: u64) -> (u64, u64) {
                let z = (x as u128).wrapping_sub(y as u128).wrapping_sub(cc as u128);
                (z as u64, ((z >> 64) as u64).wrapping_neg())
            }

            // Subtract the modulus, return borrow (compile-time).
            const fn subm(a: Self) -> (Self, u64) {
                const fn subm_inner(a: $typename, cc: u64, i: usize) -> ($typename, u64) {
                    if i == a.0.len() {
                        (a, cc)
                    } else {
                        let (d, cc) = $typename::sbb(a.0[i], $typename::MODULUS[i], cc);
                        let mut aa = a;
                        aa.0[i] = d;
                        subm_inner(aa, cc, i + 1)
                    }
                }

                subm_inner(a, 0, 0)
            }

            // Add the modulus if mm == -1; return a unchanged with mm == 0
            // (compile-time).
            const fn addm_cond(a: $typename, mm: u64) -> $typename {
                const fn addm_cond_inner(a: $typename, mm: u64, cc: u64, i: usize) -> $typename {
                    if i == a.0.len() {
                        a
                    } else {
                        let (d, cc) = $typename::adc(a.0[i], $typename::MODULUS[i] & mm, cc);
                        let mut aa = a;
                        aa.0[i] = d;
                        addm_cond_inner(aa, mm, cc, i + 1)
                    }
                }

                addm_cond_inner(a, mm, 0, 0)
            }

            // Get index of the top non-zero word of the modulus
            // (from the parameters).
            const fn top_word_index() -> usize {
                const fn top_word_index_inner(j: usize) -> usize {
                    if $modulus[j] != 0 {
                        j
                    } else {
                        top_word_index_inner(j - 1)
                    }
                }
                top_word_index_inner($modulus.len() - 1)
            }

            // Get the modulus (normalized).
            const fn make_modulus() -> [u64; Self::N] {
                const fn make_modulus_inner(
                    d: [u64; $typename::N],
                    j: usize,
                ) -> [u64; $typename::N] {
                    if j == $typename::N {
                        d
                    } else {
                        let mut dd = d;
                        dd[j] = $modulus[j];
                        make_modulus_inner(dd, j + 1)
                    }
                }
                make_modulus_inner([0u64; Self::N], 0)
            }

            // Compute the modulus exact bit length (compile-time).
            const fn mod_bitlen() -> usize {
                const fn bitlen(x: u64, max: usize) -> usize {
                    if max == 1 {
                        x as usize
                    } else {
                        let hm = max >> 1;
                        let y = x >> hm;
                        if y == 0 {
                            bitlen(x, hm)
                        } else {
                            bitlen(y, max - hm) + hm
                        }
                    }
                }
                (Self::N - 1) * 64 + bitlen($typename::MODULUS[Self::N - 1], 64)
            }

            // Get the top 32 bits of the actual modulus value (if the modulus
            // is less than 32 bits in length, then this returns the modulus).
            const fn top_u32() -> u64 {
                if Self::BIT_LENGTH < 32 {
                    Self::MODULUS[0]
                } else {
                    let hi = Self::MODULUS[Self::N - 1];
                    let bl = Self::BIT_LENGTH & 63;
                    if bl == 0 {
                        hi >> 32
                    } else if bl < 32 {
                        let lo = Self::MODULUS[Self::N - 2];
                        (hi << (32 - bl)) | (lo >> (bl + 32))
                    } else {
                        hi >> (bl - 32)
                    }
                }
            }

            // Compute 2^n in the field, using repeated additions. This is
            // used only at compile-time.
            const fn pow2mod(n: usize) -> Self {
                const fn lsh_inner(d: $typename, cc: u64, i: usize) -> ($typename, u64) {
                    if i == $typename::N {
                        (d, cc)
                    } else {
                        let mut dd = d;
                        dd.0[i] = (d.0[i] << 1) | cc;
                        let cc = d.0[i] >> 63;
                        lsh_inner(dd, cc, i + 1)
                    }
                }

                const fn pow2mod_inner(d: $typename, n: usize) -> $typename {
                    if n == 0 {
                        d
                    } else {
                        let (d, dh) = lsh_inner(d, 0, 0);
                        let (d, cc) = $typename::subm(d);
                        let d = $typename::addm_cond(d, (cc & !dh).wrapping_neg());
                        pow2mod_inner(d, n - 1)
                    }
                }

                const fn pow2mod_inner2(d: $typename, n: usize) -> $typename {
                    if n <= 25 {
                        pow2mod_inner(d, n)
                    } else {
                        pow2mod_inner2(pow2mod_inner(d, 25), n - 25)
                    }
                }

                let bl = Self::mod_bitlen();
                let mut d = Self([0u64; Self::N]);
                if n < bl {
                    d.0[n >> 6] = 1u64 << (n & 63);
                    d
                } else {
                    d.0[(bl - 1) >> 6] = 1u64 << ((bl - 1) & 63);
                    pow2mod_inner2(d, n - (bl - 1))
                }
            }

            // Const implementation of modular negation. This MUST NOT be
            // applied on zero.
            const fn const_neg(a: Self) -> Self {
                const fn const_neg_inner(
                    d: $typename,
                    a: $typename,
                    cc: u64,
                    j: usize,
                ) -> $typename {
                    if j == $typename::N {
                        d
                    } else {
                        let mut dd = d;
                        let (x, cc) = $typename::sbb($typename::MODULUS[j], a.0[j], cc);
                        dd.0[j] = x;
                        const_neg_inner(dd, a, cc, j + 1)
                    }
                }
                const_neg_inner(Self::ZERO, a, 0, 0)
            }

            // Const implementation of Montgomery multiplication. It uses
            // recursion in order to be compatible with the constraints of
            // const code; at runtime, it would be slower than the normal
            // implementation, but still constant-time (in case it gets
            // mistakenly used).
            const fn const_mmul(a: Self, b: Self) -> Self {
                const fn umaal(x: u64, y: u64, a: u64, b: u64) -> (u64, u64) {
                    let z = (x as u128) * (y as u128) + (a as u128) + (b as u128);
                    (z as u64, (z >> 64) as u64)
                }

                const fn mmul1(d: $typename, dh: u64, a: $typename, bj: u64) -> ($typename, u64) {
                    #[allow(clippy::too_many_arguments)]
                    const fn mmul1_inner(
                        d: $typename,
                        dh: u64,
                        a: $typename,
                        bj: u64,
                        fm: u64,
                        cc1: u64,
                        cc2: u64,
                        i: usize,
                    ) -> ($typename, u64) {
                        if i == d.0.len() {
                            let mut dd = d;
                            let (z, zh1) = $typename::adc(dh, cc1, 0);
                            let (z, zh2) = $typename::adc(z, cc2, 0);
                            dd.0[i - 1] = z;
                            (dd, zh1 + zh2)
                        } else {
                            let (z, cc1) = umaal(a.0[i], bj, d.0[i], cc1);
                            let (z, cc2) = umaal($typename::MODULUS[i], fm, z, cc2);
                            let mut dd = d;
                            if i > 0 {
                                dd.0[i - 1] = z;
                            }
                            mmul1_inner(dd, dh, a, bj, fm, cc1, cc2, i + 1)
                        }
                    }

                    let fm = a.0[0]
                        .wrapping_mul(bj)
                        .wrapping_add(d.0[0])
                        .wrapping_mul($typename::P0I);
                    mmul1_inner(d, dh, a, bj, fm, 0, 0, 0)
                }

                const fn mmul_inner(
                    d: $typename,
                    dh: u64,
                    a: $typename,
                    b: $typename,
                    j: usize,
                ) -> ($typename, u64) {
                    if j == d.0.len() {
                        (d, dh)
                    } else {
                        let (d, dh) = mmul1(d, dh, a, b.0[j]);
                        mmul_inner(d, dh, a, b, j + 1)
                    }
                }

                let (d, dh) = mmul_inner(Self([0u64; $typename::N]), 0, a, b, 0);
                let (d, cc) = $typename::subm(d);
                $typename::addm_cond(d, (cc & !dh).wrapping_neg())
            }

            const fn const_rev(x: [u64; Self::N]) -> [u64; Self::N] {
                const fn const_rev_inner(x: [u64; $typename::N], i: usize) -> [u64; $typename::N] {
                    let j = $typename::N - 1 - i;
                    if j <= i {
                        x
                    } else {
                        let mut y = x;
                        y[i] = x[j];
                        y[j] = x[i];
                        const_rev_inner(y, i + 1)
                    }
                }
                const_rev_inner(x, 0)
            }

            const fn const_small(x: u64) -> Self {
                let mut d = [0u64; Self::N];
                d[0] = x;
                Self::const_mmul(Self(d), Self::R2)
            }

            const fn const_sqrt_exp() -> [u64; Self::N] {
                const fn const_sqrt_exp_inner(
                    d: [u64; $typename::N],
                    cc: u64,
                    dd: u64,
                    i: usize,
                ) -> [u64; $typename::N] {
                    if i == $typename::N {
                        let mut d2 = d;
                        d2[$typename::N - 1] = dd;
                        d2
                    } else {
                        let (x, cc) = $typename::adc($typename::MODULUS[i], 0, cc);
                        let mut d2 = d;
                        if i > 0 {
                            d2[i - 1] = dd | (x << 62);
                        }
                        const_sqrt_exp_inner(d2, cc, x >> 2, i + 1)
                    }
                }
                const_sqrt_exp_inner([0u64; Self::N], 1, 0, 0)
            }

            const fn const_fourth_root_exp() -> [u64; Self::N] {
                const fn const_fourth_root_exp_inner(
                    d: [u64; $typename::N],
                    cc: u64,
                    dd: u64,
                    i: usize,
                ) -> [u64; $typename::N] {
                    if i == $typename::N {
                        let mut d2 = d;
                        d2[$typename::N - 1] = dd;
                        d2
                    } else {
                        let (x, cc) = $typename::adc($typename::MODULUS[i], 0, cc);
                        let mut d2 = d;
                        if i > 0 {
                            d2[i - 1] = dd | (x << 61);
                        }
                        const_fourth_root_exp_inner(d2, cc, x >> 3, i + 1)
                    }
                }
                const_fourth_root_exp_inner([0u64; Self::N], 1, 0, 0)
            }

            /// Decode an element from bytes, no check is made that the input
            /// value is reduced except that the buffer is of the excpected
            /// length of `Self::ENCODED_LENGTH`.
            pub const fn const_decode_no_check(buf: &[u8]) -> Self {
                let mut r = Self::ZERO;
                if buf.len() != Self::ENCODED_LENGTH {
                    return r;
                }

                // Fill the first N-1 elements
                let mut i = 0;
                while i < Self::N - 1 {
                    r.0[i] = u64::from_le_bytes([
                        buf[i * 8],
                        buf[i * 8 + 1],
                        buf[i * 8 + 2],
                        buf[i * 8 + 3],
                        buf[i * 8 + 4],
                        buf[i * 8 + 5],
                        buf[i * 8 + 6],
                        buf[i * 8 + 7],
                    ]);
                    i += 1;
                }

                // Fill the last element
                let mut w = 0u64;
                let mut j = 0;
                while j < Self::ENCODED_LENGTH - (Self::N - 1) * 8 {
                    w |= (buf[(Self::N - 1) * 8 + j] as u64) << (8 * j);
                    j += 1;
                }
                r.0[Self::N - 1] = w;

                Self::const_mmul(r, Self::R2)
            }
        }

        /*
         * Implementations of all the traits needed to use the simple operators
         * (+, *, /...) on field element instances, with or without references.
         * as well as a display method.
         */

        impl ::std::fmt::Display for $typename {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // Encode the value to get a canonical value
                let v_bytes = self.encode();

                // Convert the bytes into a base-16 string
                let mut v_hex_string: String = v_bytes
                    .iter()
                    .rev()
                    .map(|byte| format!("{:02x}", byte)) // Format each byte as a two-digit hex
                    .collect();

                // Remove leading zeros
                v_hex_string = v_hex_string.trim_start_matches("0").to_string();

                // If the value was zero, we need to add a zero back
                if v_hex_string.is_empty() {
                    v_hex_string = String::from("0");
                }

                write!(f, "0x{}", v_hex_string)
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
    };
} // End of macro: define_fp_core
