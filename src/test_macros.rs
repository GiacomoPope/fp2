//! Macros to generate deterministically random test vectors for finite fields

/// A macro to generate test vectors for a given finite field Fp.
///
/// Macro expectations:
/// - $Fp: a finite field type of degree 1
#[cfg_attr(feature = "test-utils", macro_export)]
#[cfg_attr(not(feature = "test-utils"), allow(unused_macros))]
macro_rules! define_fp_tests {
    ($Fp:ty) => {
        use ::num_bigint::ToBigInt as _;
        use ::sha2::Digest as _;

        // ----------------------------------------------------------------
        // Shared test-data helpers
        // ----------------------------------------------------------------

        /// Build the reference modulus as a `BigInt`.
        ///
        /// NOTE: `num_bigint::BigInt::from_slice` takes `&[u32]`, so each u64
        /// limb is split into its low and high 32-bit halves.
        fn fp_modulus() -> ::num_bigint::BigInt {
            let mut zp_u32_words = [0u32; <$Fp>::N * 2];
            for i in 0..<$Fp>::N {
                zp_u32_words[2 * i] = <$Fp>::MODULUS[i] as u32;
                zp_u32_words[2 * i + 1] = (<$Fp>::MODULUS[i] >> 32) as u32;
            }
            ::num_bigint::BigInt::from_slice(::num_bigint::Sign::Plus, &zp_u32_words)
        }

        /// Generate a single deterministic test byte vector for index `i`.
        fn fp_test_vector(i: usize) -> Vec<u8> {
            let len = (<$Fp>::ENCODED_LENGTH + 64) & !31usize;
            let mut fp_bytes = vec![0u8; len];
            let mut sh = ::sha2::Sha256::new();
            for j in 0..(len >> 5) {
                sh.update([i as u64, j as u64].map(u64::to_le_bytes).concat());
                fp_bytes[(32 * j)..(32 * j + 32)].copy_from_slice(&sh.finalize_reset());
            }
            fp_bytes
        }

        /// Return a zeroed byte vector of the standard test length,
        /// representing the field element zero.
        fn fp_zero_vector() -> Vec<u8> {
            vec![0u8; (<$Fp>::ENCODED_LENGTH + 64) & !31usize]
        }

        /// `encode` / `decode_reduce`: round-trip and canonical reduction.
        #[test]
        fn fp_test_encode_decode() {
            let zp = fp_modulus();

            // Random elements: decode_reduce then encode must give za % p.
            for i in 0..500 {
                let va = fp_test_vector(i);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &<$Fp>::decode_reduce(&va).encode(),
                );
                assert_eq!(
                    zc,
                    &za % &zp,
                    "iter {i}: encode/decode_reduce round-trip failed"
                );
            }

            // Zero bytes must decode to the zero element and re-encode to zero bytes.
            let zero = <$Fp>::decode_reduce(&fp_zero_vector());
            assert_eq!(
                zero.is_zero(),
                u32::MAX,
                "decode_reduce(0) should give the zero element"
            );
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &zero.encode());
            assert_eq!(
                zc,
                ::num_bigint::BigInt::ZERO,
                "zero element must encode to zero bytes"
            );
        }

        /// `decode` (strict): rejects non-canonical encodings and wrong lengths.
        #[test]
        fn fp_test_decode_strict() {
            // A valid encoding must round-trip successfully.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let encoded = a.encode();
            let (b, ok) = <$Fp>::decode(&encoded);
            assert_eq!(ok, u32::MAX, "decode of valid encoding failed");
            assert_eq!(a.equals(&b), u32::MAX, "decode round-trip value mismatch");

            // Zero is a valid encoding and must round-trip to the zero element.
            let (zero, ok) = <$Fp>::decode(&vec![0u8; <$Fp>::ENCODED_LENGTH]);
            assert_eq!(ok, u32::MAX, "decode of zero encoding should succeed");
            assert_eq!(
                zero.is_zero(),
                u32::MAX,
                "decode of zero bytes should give zero element"
            );

            // Wrong length must fail and return zero.
            let (zero, ok) = <$Fp>::decode(&encoded[..encoded.len() - 1]);
            assert_eq!(ok, 0x00000000, "decode of short buffer should fail");
            assert_eq!(
                zero.is_zero(),
                u32::MAX,
                "failed decode should return zero element"
            );

            // The modulus itself encodes a value >= p and must fail.
            let mut p_bytes = vec![0u8; <$Fp>::ENCODED_LENGTH];
            for i in 0..<$Fp>::N {
                let word_bytes = <$Fp>::MODULUS[i].to_le_bytes();
                let start = i * 8;
                let end = (start + 8).min(<$Fp>::ENCODED_LENGTH);
                p_bytes[start..end].copy_from_slice(&word_bytes[..(end - start)]);
            }
            let (zero, ok) = <$Fp>::decode(&p_bytes);
            assert_eq!(ok, 0x00000000, "decode of p (>= modulus) should fail");
            assert_eq!(
                zero.is_zero(),
                u32::MAX,
                "failed decode should return zero element"
            );
        }

        /// Addition: `(a + b) mod p`.
        #[test]
        fn fp_test_add() {
            let zp = fp_modulus();

            // Random pairs.
            for i in 0..500 {
                let va = fp_test_vector(2 * i);
                let vb = fp_test_vector(2 * i + 1);
                let a = <$Fp>::decode_reduce(&va);
                let b = <$Fp>::decode_reduce(&vb);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zb = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vb);
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &(a + b).encode(),
                );
                assert_eq!(zc, (&za + &zb) % &zp, "iter {i}: addition failed");
            }

            // a + 0 == a.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            assert_eq!((a + z).equals(&a), u32::MAX, "a + 0 should equal a");

            // 0 + 0 == 0.
            assert_eq!((z + z).is_zero(), u32::MAX, "0 + 0 should be zero");
        }

        /// Subtraction: `(a - b) mod p`.
        #[test]
        fn fp_test_sub() {
            let zp = fp_modulus();
            let zpz = &zp << 64;

            // Random pairs.
            for i in 0..500 {
                let va = fp_test_vector(2 * i);
                let vb = fp_test_vector(2 * i + 1);
                let a = <$Fp>::decode_reduce(&va);
                let b = <$Fp>::decode_reduce(&vb);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zb = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vb);
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &(a - b).encode(),
                );
                assert_eq!(
                    zc,
                    ((&zpz + &za) - (&zb % &zp)) % &zp,
                    "iter {i}: subtraction failed"
                );
            }

            // a - 0 == a.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            assert_eq!((a - z).equals(&a), u32::MAX, "a - 0 should equal a");

            // : a - a == 0.
            assert_eq!((a - a).is_zero(), u32::MAX, "a - a should be zero");

            // 0 - 0 == 0.
            assert_eq!((z - z).is_zero(), u32::MAX, "0 - 0 should be zero");
        }

        /// Negation: `-a mod p`.
        #[test]
        fn fp_test_neg() {
            let zp = fp_modulus();

            // Random elements.
            for i in 0..500 {
                let va = fp_test_vector(i);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zc =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &(-a).encode());
                assert_eq!(zc, (&zp - (&za % &zp)) % &zp, "iter {i}: negation failed");
            }

            // -0 == 0.
            let z = <$Fp>::ZERO;
            assert_eq!((-z).is_zero(), u32::MAX, "-0 should be zero");
        }

        /// Multiplication: `(a * b) mod p`.
        #[test]
        fn fp_test_mul() {
            let zp = fp_modulus();

            // Random pairs.
            for i in 0..500 {
                let va = fp_test_vector(2 * i);
                let vb = fp_test_vector(2 * i + 1);
                let a = <$Fp>::decode_reduce(&va);
                let b = <$Fp>::decode_reduce(&vb);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zb = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vb);
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &(a * b).encode(),
                );
                assert_eq!(zc, (&za * &zb) % &zp, "iter {i}: multiplication failed");
            }

            // a * 0 == 0 and 0 * 0 == 0.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            assert_eq!((a * z).is_zero(), u32::MAX, "a * 0 should be zero");
            assert_eq!((z * z).is_zero(), u32::MAX, "0 * 0 should be zero");
        }

        /// Squaring: `a^2 mod p`, and `square() == a * a`.
        #[test]
        fn fp_test_square() {
            let zp = fp_modulus();

            // Random elements.
            for i in 0..500 {
                let va = fp_test_vector(i);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &a.square().encode(),
                );
                assert_eq!(zc, (&za * &za) % &zp, "iter {i}: squaring failed");
                assert_eq!(
                    a.square().equals(&(a * a)),
                    u32::MAX,
                    "iter {i}: square() != a * a"
                );
            }

            // 0^2 == 0.
            let z = <$Fp>::ZERO;
            assert_eq!(z.square().is_zero(), u32::MAX, "0^2 should be zero");
        }

        /// N-squaring: `n_square(n)` matches repeated `square()` n times.
        #[test]
        fn fp_test_n_square() {
            // Random elements.
            for i in 0..100 {
                let a = <$Fp>::decode_reduce(&fp_test_vector(i));
                for n in [1u32, 2, 5, 10, 20] {
                    let mut b = a;
                    for _ in 0..n {
                        b = b.square();
                    }
                    let c = a.n_square(n);
                    assert_eq!(c.equals(&b), u32::MAX, "iter {i}: n_square(n) failed");
                }
            }
            // 0^(2^n) == 0.
            let z = <$Fp>::decode_reduce(&fp_zero_vector());
            for n in [1u32, 2, 5] {
                let c = z.n_square(n);
                assert_eq!(c.is_zero(), u32::MAX, "n={n}: 0^(2^n) should be zero");
            }
        }

        /// Halving: `half(a) + half(a) == a mod p`.
        #[test]
        fn fp_test_half() {
            let zp = fp_modulus();

            // Random elements.
            for i in 0..500 {
                let va = fp_test_vector(i);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let mut c = a.half();
                c += c;
                let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &c.encode());
                assert_eq!(zc, &za % &zp, "iter {i}: half failed");
            }

            // 0 / 2 == 0.
            let z = <$Fp>::ZERO;
            assert_eq!(z.half().is_zero(), u32::MAX, "0/2 should be zero");
        }

        /// Small multiples: `mul2`, `mul3`, `mul4`.
        #[test]
        fn fp_test_small_multiples() {
            let zp = fp_modulus();

            // Random elements.
            for i in 0..500 {
                let va = fp_test_vector(i);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);

                let zc2 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &a.mul2().encode(),
                );
                assert_eq!(zc2, (&za + &za) % &zp, "iter {i}: mul2 failed");

                let zc3 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &a.mul3().encode(),
                );
                assert_eq!(zc3, (&za + &za + &za) % &zp, "iter {i}: mul3 failed");

                let zc4 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &a.mul4().encode(),
                );
                assert_eq!(zc4, (&za + &za + &za + &za) % &zp, "iter {i}: mul4 failed");
            }

            // mul2/3/4 of zero must be zero.
            let z = <$Fp>::ZERO;
            assert_eq!(z.mul2().is_zero(), u32::MAX, "2 * 0 should be zero");
            assert_eq!(z.mul3().is_zero(), u32::MAX, "3 * 0 should be zero");
            assert_eq!(z.mul4().is_zero(), u32::MAX, "4 * 0 should be zero");
        }

        /// `mul_small`: multiply by a small signed `i32`.
        #[test]
        fn fp_test_mul_small() {
            let zp = fp_modulus();
            let zpz = &zp << 64;

            // Random pairs: use a second vector as the source of the scalar k.
            for i in 0..500 {
                let va = fp_test_vector(2 * i);
                let vb = fp_test_vector(2 * i + 1);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let k = i32::from_le_bytes(vb[0..4].try_into().unwrap());
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &a.mul_small(k).encode(),
                );
                assert_eq!(
                    zc,
                    ((&za % &zp) * k + &zpz) % &zp,
                    "iter {i}: mul_small failed"
                );
            }

            // Edge case: i32::MIN — |i32::MIN| fits in u32 but the sign path
            // in the implementation is non-obvious.
            let va = fp_test_vector(0);
            let a = <$Fp>::decode_reduce(&va);
            let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
            let zc = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &a.mul_small(i32::MIN).encode(),
            );
            assert_eq!(
                zc,
                ((&za % &zp) * i32::MIN + &zpz) % &zp,
                "mul_small(i32::MIN) failed"
            );

            // a * 0 == 0 and 0 * k == 0.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            assert_eq!(
                a.mul_small(0).is_zero(),
                u32::MAX,
                "mul_small(a, 0) should be zero"
            );
            assert_eq!(
                z.mul_small(163).is_zero(),
                u32::MAX,
                "mul_small(0, k) should be zero"
            );
        }

        /// `from_i32`, `from_u32`, `from_i64`, `from_u64`.
        #[test]
        fn fp_test_from_integer() {
            let zp = fp_modulus();

            // Random byte sources used as raw integer inputs.
            for i in 0..500 {
                let va = fp_test_vector(i);
                let k32 = i32::from_le_bytes(va[0..4].try_into().unwrap());
                let ku32 = k32 as u32;
                let k64 = i64::from_le_bytes(va[0..8].try_into().unwrap());
                let ku64 = k64 as u64;

                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &<$Fp>::from(k32).encode(),
                );
                assert_eq!(
                    zc,
                    (k32.to_bigint().unwrap() + &zp) % &zp,
                    "iter {i}: from_i32 failed"
                );

                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &<$Fp>::from(ku32).encode(),
                );
                assert_eq!(zc, ku32.to_bigint().unwrap(), "iter {i}: from_u32 failed");

                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &<$Fp>::from(k64).encode(),
                );
                assert_eq!(
                    zc,
                    (k64.to_bigint().unwrap() + &zp) % &zp,
                    "iter {i}: from_i64 failed"
                );

                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &<$Fp>::from(ku64).encode(),
                );
                assert_eq!(zc, ku64.to_bigint().unwrap(), "iter {i}: from_u64 failed");
            }

            // Zero inputs must give the zero element.
            assert_eq!(
                <$Fp>::from(0i32).is_zero(),
                u32::MAX,
                "from(0i32) should be zero"
            );
            assert_eq!(
                <$Fp>::from(0u32).is_zero(),
                u32::MAX,
                "from(0u32) should be zero"
            );
            assert_eq!(
                <$Fp>::from(0i64).is_zero(),
                u32::MAX,
                "from(0i64) should be zero"
            );
            assert_eq!(
                <$Fp>::from(0u32).is_zero(),
                u32::MAX,
                "from(0u32) should be zero"
            );
        }

        /// Division / inversion: `(a / b) * b == a`; `a / 0 == 0`.
        #[test]
        fn fp_test_div() {
            let zp = fp_modulus();

            // Random pairs (heuristically, all non-zero since fp_test_vector is pseudorandom).
            for i in 0..500 {
                let va = fp_test_vector(2 * i);
                let vb = fp_test_vector(2 * i + 1);
                let a = <$Fp>::decode_reduce(&va);
                let b = <$Fp>::decode_reduce(&vb);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);
                let zc = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &(a / b * b).encode(),
                );
                assert_eq!(zc, &za % &zp, "iter {i}: division round-trip failed");
            }

            // Division by zero: a / 0 == 0 and 0 / 0 == 0.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            assert_eq!((a / z).is_zero(), u32::MAX, "a / 0 should be zero");
            assert_eq!((z / z).is_zero(), u32::MAX, "0 / 0 should be zero");

            // Zero numerator: 0 / a == 0 for non-zero a.
            assert_eq!((z / a).is_zero(), u32::MAX, "0 / a should be zero");
        }

        /// `batch_invert`: agrees with individual `invert()` calls, including
        /// when the slice contains zeros.
        #[test]
        fn fp_test_batch_invert() {
            // Non-zero batch: each result must match individual invert().
            let n = 20;
            let mut elems: Vec<_> = (0..n)
                .map(|j| <$Fp>::decode_reduce(&fp_test_vector(j)))
                .collect();
            let expected: Vec<_> = elems.iter().map(|x| x.invert()).collect();
            <$Fp>::batch_invert(&mut elems);
            for (j, (got, exp)) in elems.iter().zip(expected.iter()).enumerate() {
                assert_eq!(
                    got.equals(exp),
                    u32::MAX,
                    "batch_invert elem {j}: disagrees with invert()"
                );
            }

            // Batch containing zeros at known positions: invert(0) == 0 by
            // convention, so batch_invert must return zero at those positions.
            let mut mixed: Vec<_> = (0..10)
                .map(|j| <$Fp>::decode_reduce(&fp_test_vector(j)))
                .collect();
            mixed[3] = <$Fp>::ZERO;
            mixed[7] = <$Fp>::ZERO;
            let expected: Vec<_> = mixed.iter().map(|x| x.invert()).collect();
            <$Fp>::batch_invert(&mut mixed);
            for (j, (got, exp)) in mixed.iter().zip(expected.iter()).enumerate() {
                assert_eq!(
                    got.equals(exp),
                    u32::MAX,
                    "batch_invert (with zeros) elem {j}: disagrees with invert()"
                );
            }
        }

        /// `pow` / Fermat's little theorem: `a^(p-1) == 1` for all non-zero `a`.
        #[test]
        fn fp_test_pow_fermat() {
            // Compute p-1 as a little-endian byte array.
            let mut exp_bytes = vec![0u8; <$Fp>::N * 8];
            let mut borrow = 1u64;
            for i in 0..<$Fp>::N {
                let (d, b) = <$Fp>::MODULUS[i].overflowing_sub(borrow);
                exp_bytes[i * 8..(i + 1) * 8].copy_from_slice(&d.to_le_bytes());
                borrow = b as u64;
            }
            // p-1 has the same bit length as p for all odd primes p.
            let ebitlen = <$Fp>::BIT_LENGTH;

            // Random non-zero elements must satisfy Fermat's little theorem.
            for i in 0..20 {
                let a = <$Fp>::decode_reduce(&fp_test_vector(i));
                assert_eq!(
                    a.pow(&exp_bytes, ebitlen).equals(&<$Fp>::ONE),
                    u32::MAX,
                    "iter {i}: a^(p-1) != 1 (Fermat's little theorem)"
                );
            }

            // Zero: the field convention defines 0^(p-1) as zero, not one.
            let z = <$Fp>::ZERO;
            assert_eq!(
                z.pow(&exp_bytes, ebitlen).is_zero(),
                u32::MAX,
                "0^(p-1) should be zero under the field convention"
            );
        }

        #[test]
        fn fp_test_pow_fermat_pubexp() {
            // Compute p-1 as a little-endian byte array.
            let mut pm1 = <$Fp>::MODULUS;
            pm1[0] -= 1;

            // p-1 has the same bit length as p for all odd primes p.
            let ebitlen = <$Fp>::BIT_LENGTH;

            // Random non-zero elements must satisfy Fermat's little theorem.
            for i in 0..20 {
                let a = <$Fp>::decode_reduce(&fp_test_vector(i));
                assert_eq!(
                    a.pow_pubexp(&pm1).equals(&<$Fp>::ONE),
                    u32::MAX,
                    "iter {i}: a^(p-1) != 1 (Fermat's little theorem)"
                );
            }

            // Zero: the field convention defines 0^(p-1) as zero, not one.
            let z = <$Fp>::ZERO;
            assert_eq!(
                z.pow_pubexp(&pm1).is_zero(),
                u32::MAX,
                "0^(p-1) should be zero under the field convention"
            );
        }

        /// `select`, `set_cond`, `cond_swap`, `set_cond_neg`.
        #[test]
        fn fp_test_conditional_ops() {
            // Random pairs.
            for i in 0..100 {
                let a = <$Fp>::decode_reduce(&fp_test_vector(2 * i));
                let b = <$Fp>::decode_reduce(&fp_test_vector(2 * i + 1));

                // select: ctl=0 returns a, ctl=MAX returns b.
                assert_eq!(
                    <$Fp>::select(&a, &b, 0x00000000).equals(&a),
                    u32::MAX,
                    "iter {i}: select(a,b,0) should be a"
                );
                assert_eq!(
                    <$Fp>::select(&a, &b, u32::MAX).equals(&b),
                    u32::MAX,
                    "iter {i}: select(a,b,MAX) should be b"
                );

                // set_cond: unchanged on 0, overwritten on MAX.
                let mut c = a;
                c.set_cond(&b, 0x00000000);
                assert_eq!(
                    c.equals(&a),
                    u32::MAX,
                    "iter {i}: set_cond(0) should not change value"
                );
                c.set_cond(&b, u32::MAX);
                assert_eq!(
                    c.equals(&b),
                    u32::MAX,
                    "iter {i}: set_cond(0xFF..FF) should overwrite"
                );

                // cond_swap: no swap on 0, swap on MAX.
                let mut x = a;
                let mut y = b;
                <$Fp>::cond_swap(&mut x, &mut y, 0x00000000);
                assert_eq!(x.equals(&a), u32::MAX, "iter {i}: cond_swap(0) changed x");
                assert_eq!(y.equals(&b), u32::MAX, "iter {i}: cond_swap(0) changed y");
                <$Fp>::cond_swap(&mut x, &mut y, u32::MAX);
                assert_eq!(
                    x.equals(&b),
                    u32::MAX,
                    "iter {i}: cond_swap(0xFF..FF) x should be old y"
                );
                assert_eq!(
                    y.equals(&a),
                    u32::MAX,
                    "iter {i}: cond_swap(0xFF..FF) y should be old x"
                );

                // set_cond_neg: unchanged on 0, negated on MAX.
                let mut c = a;
                c.set_cond_neg(0x00000000);
                assert_eq!(
                    c.equals(&a),
                    u32::MAX,
                    "iter {i}: set_cond_neg(0) should not negate"
                );
                c.set_cond_neg(u32::MAX);
                assert_eq!(
                    c.equals(&(-a)),
                    u32::MAX,
                    "iter {i}: set_cond_neg(0xFF..FF) should negate"
                );
            }

            // Ops involving zero: selecting and swapping with zero must be correct.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            assert_eq!(
                <$Fp>::select(&z, &a, 0x00000000).is_zero(),
                u32::MAX,
                "select(0, a, 0) should be zero"
            );
            assert_eq!(
                <$Fp>::select(&z, &a, u32::MAX).equals(&a),
                u32::MAX,
                "select(0, a, 0xFF..FF) should be a"
            );

            // set_cond_neg of zero is still zero (negation of 0 is 0).
            let mut z = <$Fp>::ZERO;
            z.set_cond_neg(u32::MAX);
            assert_eq!(
                z.is_zero(),
                u32::MAX,
                "set_cond_neg(0xFF..FF) of zero should still be zero"
            );
        }

        /// Legendre symbol and `is_square`.
        #[test]
        fn fp_test_legendre_and_is_square() {
            // Random elements: a^2 is always a QR, -(a^2) is always a non-QR
            // (for non-zero a).
            for i in 0..250 {
                let a = <$Fp>::decode_reduce(&fp_test_vector(i));
                let sq = a.square();
                if sq.is_zero() == u32::MAX {
                    assert_eq!(sq.legendre(), 0, "iter {i}: legendre(0) should be 0");
                    assert_eq!(
                        sq.is_square(),
                        u32::MAX,
                        "iter {i}: is_square(0) should be true"
                    );
                } else {
                    assert_eq!(sq.legendre(), 1, "iter {i}: legendre(a^2) should be 1");
                    assert_eq!(
                        sq.is_square(),
                        u32::MAX,
                        "iter {i}: is_square(a^2) should be true"
                    );
                    assert_eq!(
                        (-sq).legendre(),
                        -1,
                        "iter {i}: legendre(-a^2) should be -1"
                    );
                    assert_eq!(
                        (-sq).is_square(),
                        0,
                        "iter {i}: is_square(-a^2) should be false"
                    );
                }
            }

            // Zero: legendre(0) == 0 and zero is considered a square.
            let z = <$Fp>::ZERO;
            assert_eq!(z.legendre(), 0, "legendre(0) should be 0");
            assert_eq!(z.is_square(), u32::MAX, "is_square(0) should be true");
        }

        /// `sum_of_products` and `difference_of_products`.
        #[test]
        fn fp_test_sum_and_difference_of_products() {
            // Random pairs.
            for i in 0..500 {
                let a = <$Fp>::decode_reduce(&fp_test_vector(4 * i));
                let b = <$Fp>::decode_reduce(&fp_test_vector(4 * i + 1));
                let c = <$Fp>::decode_reduce(&fp_test_vector(4 * i + 2));
                let d = <$Fp>::decode_reduce(&fp_test_vector(4 * i + 3));

                let expected_sum = a * b + c * d;
                let expected_diff = a * b - c * d;

                assert_eq!(
                    <$Fp>::sum_of_products(&a, &b, &c, &d).equals(&expected_sum),
                    u32::MAX,
                    "iter {i}: sum_of_products failed",
                );
                assert_eq!(
                    <$Fp>::difference_of_products(&a, &b, &c, &d).equals(&expected_diff),
                    u32::MAX,
                    "iter {i}: difference_of_products failed",
                );
                // difference_of_products must agree with sum_of_products with a negated last factor.
                assert_eq!(
                    <$Fp>::sum_of_products(&a, &b, &c, &(-&d)).equals(&expected_diff),
                    u32::MAX,
                    "iter {i}: sum_of_products with -d should equal difference_of_products",
                );
            }

            // Zero factor: sum_of_products(a, 0, c, d) reduces to c * d.
            let a = <$Fp>::decode_reduce(&fp_test_vector(0));
            let z = <$Fp>::ZERO;
            let c = <$Fp>::decode_reduce(&fp_test_vector(1));
            let d = <$Fp>::decode_reduce(&fp_test_vector(2));
            assert_eq!(
                <$Fp>::sum_of_products(&a, &z, &c, &d).equals(&(c * d)),
                u32::MAX,
                "sum_of_products(a, 0, c, d) should equal c * d",
            );
        }

        /// Square roots: `sqrt(a^2)` succeeds; `sqrt(-a^2)` fails; result LSB is zero.
        #[test]
        fn fp_test_sqrt() {
            let zp = fp_modulus();

            // Random elements.
            for i in 0..30 {
                let va = fp_test_vector(i);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);

                // sqrt(a^2) must succeed and satisfy (sqrt)^2 == a^2 mod p.
                let (c, r) = (a * a).sqrt();
                assert_eq!(r, u32::MAX, "iter {i}: sqrt of square failed");
                let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &c.encode());
                assert_eq!(
                    (&zc * &zc) % &zp,
                    (&za * &za) % &zp,
                    "iter {i}: sqrt(a^2)^2 != a^2"
                );

                // -(a^2) is not a square for non-zero a; sqrt must fail and return zero.
                if a.is_zero() == 0 {
                    let (c, r) = (-(a * a)).sqrt();
                    assert_eq!(r, 0x00000000, "iter {i}: sqrt of non-square should fail");
                    assert_eq!(
                        c.is_zero(),
                        u32::MAX,
                        "iter {i}: failed sqrt should return zero"
                    );
                }
            }

            // sqrt(0) == 0: zero is its own square root.
            let z = <$Fp>::ZERO;
            let (c, r) = z.sqrt();
            assert_eq!(r, u32::MAX, "sqrt(0) should succeed");
            assert_eq!(c.is_zero(), u32::MAX, "sqrt(0) should return zero");
        }

        /// Fourth roots: `fourth_root(a^4)` succeeds; `sqrt(-(a^4))` fails; result LSB is zero.
        #[test]
        fn fp_test_fourth_root() {
            let zp = fp_modulus();

            // Random elements.
            for i in 0..30 {
                let va = fp_test_vector(i);
                let a = <$Fp>::decode_reduce(&va);
                let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &va);

                // fourth_root(a^4) must succeed and satisfy (root)^4 == a^4 mod p.
                let (c, r) = (a * a * a * a).fourth_root();
                assert_eq!(r, u32::MAX, "iter {i}: fourth_root of fourth-power failed");
                let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &c.encode());
                assert_eq!(
                    (&zc * &zc * &zc * &zc) % &zp,
                    (&za * &za * &za * &za) % &zp,
                    "iter {i}: fourth_root(a^4)^4 != a^4"
                );

                // -(a^4) has no fourth root for non-zero a; sqrt must fail and return zero.
                if a.is_zero() == 0 {
                    let (c, r) = (-(a * a * a * a)).sqrt();
                    assert_eq!(r, 0x00000000, "iter {i}: sqrt of -(a^4) should fail");
                    assert_eq!(
                        c.is_zero(),
                        u32::MAX,
                        "iter {i}: failed sqrt should return zero"
                    );
                }
            }

            // fourth_root(0) == 0: zero is its own fourth root.
            let z = <$Fp>::ZERO;
            let (c, r) = z.fourth_root();
            assert_eq!(r, u32::MAX, "fourth_root(0) should succeed");
            assert_eq!(c.is_zero(), u32::MAX, "fourth_root(0) should return zero");
        }

        #[test]
        fn test_fp_trait_static_methods() {
            use fp2::traits::Fq;
            fn via_trait<F: Fq>(x: F, y: F) {
                let mut elems = [F::ONE, F::TWO, F::THREE];
                F::batch_invert(&mut elems);
                let _ = F::select(&F::ZERO, &F::ONE, 0);
                let _ = F::decode(&[0u8; 8]);
                let _ = F::decode_reduce(&[0u8; 8]);
                // instance methods
                let _ = x.invert();
                let inv = x.invert();
                assert_eq!((inv * x).equals(&F::ONE), 0xFFFFFFFF);
                let (root, ok) = x.square().sqrt();
                assert_eq!(ok, 0xFFFFFFFF);
                assert_eq!(root.square().equals(&x.square()), 0xFFFFFFFF);
            }
            via_trait(<$Fp>::from(3u32), <$Fp>::from(7u32));
        }
    };
} // End of macro: define_fp_tests

/// A macro to generate test vectors for a given finite field Fp^2.
///
/// Macro expectations:
/// - $Fp2: a degree two extension Fp^2 of the finite field Fp
/// - $modulus: the base-field modulus as a `[u64; N]` literal
/// - $nqr: a `u64` such that `$nqr + i` is a non-quadratic residue in Fp^2
#[cfg_attr(feature = "test-utils", macro_export)]
#[cfg_attr(not(feature = "test-utils"), allow(unused_macros))]
macro_rules! define_fp2_tests {
    ($Fp2:ty, $modulus:expr, $nqr:literal) => {
        use ::num_bigint::ToBigInt as _;
        use ::sha2::Digest as _;

        // ----------------------------------------------------------------
        // Shared test-data helpers
        // ----------------------------------------------------------------

        /// Number of u64 words in the base-field characteristic.
        pub static N: usize = $modulus.len();

        /// Byte length of one base-field element (half of the Fp2 encoding).
        pub static FP_ENCODED_LENGTH: usize = <$Fp2>::ENCODED_LENGTH >> 1;

        /// Build the reference base-field modulus p as a `BigInt`.
        ///
        /// NOTE: `num_bigint::BigInt::from_slice` takes `&[u32]`, so each u64
        /// limb is split into its low and high 32-bit halves.
        fn fp2_modulus() -> ::num_bigint::BigInt {
            let mut zp_u32_words = [0u32; $modulus.len() * 2];
            for i in 0..$modulus.len() {
                zp_u32_words[2 * i] = $modulus[i] as u32;
                zp_u32_words[2 * i + 1] = ($modulus[i] >> 32) as u32;
            }
            ::num_bigint::BigInt::from_slice(::num_bigint::Sign::Plus, &zp_u32_words)
        }

        /// Generate a single deterministic test byte vector for index `i`.
        fn fp2_test_vector(i: usize) -> Vec<u8> {
            let len = (<$Fp2>::ENCODED_LENGTH + 64) & !31usize;
            let mut fp2_bytes = vec![0u8; len];
            let mut sh = ::sha2::Sha256::new();
            for j in 0..(len >> 5) {
                sh.update([i as u64, j as u64].map(u64::to_le_bytes).concat());
                fp2_bytes[(32 * j)..(32 * j + 32)].copy_from_slice(&sh.finalize_reset());
            }
            fp2_bytes
        }

        /// Return a zeroed byte vector of the standard Fp2 test length,
        /// representing the field element zero.
        fn fp2_zero_vector() -> Vec<u8> {
            vec![0u8; (<$Fp2>::ENCODED_LENGTH + 64) & !31usize]
        }

        /// Decode the two base-field components from an encoded Fp2 element.
        fn fp2_decode_components(vc: &[u8]) -> (::num_bigint::BigInt, ::num_bigint::BigInt) {
            let mid = FP_ENCODED_LENGTH;
            let z0 = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc[..mid]);
            let z1 = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc[mid..]);
            (z0, z1)
        }

        /// Construct a known non-quadratic-residue in Fp2:
        ///   nqr = $nqr + i   (real part $nqr, imaginary part 1)
        fn fp2_nqr() -> $Fp2 {
            let mut nqr: $Fp2 = <$Fp2>::from($nqr);
            nqr += <$Fp2>::ZETA;
            nqr
        }

        // ----------------------------------------------------------------
        // Individual tests
        // ----------------------------------------------------------------

        /// `encode` / `decode_reduce`: both components are reduced mod p.
        #[test]
        fn fp2_test_encode_decode() {
            let zp = fp2_modulus();

            // Random elements.
            for i in 0..100 {
                let va = fp2_test_vector(i);
                let a = <$Fp2>::decode_reduce(&va);
                let (zc0, zc1) = fp2_decode_components(&a.encode());
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                assert_eq!(zc0, &za0 % &zp, "iter {i}: x0 encode/decode_reduce failed");
                assert_eq!(zc1, &za1 % &zp, "iter {i}: x1 encode/decode_reduce failed");
            }

            // Zero bytes must decode to the zero element.
            let zero = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!(
                zero.is_zero(),
                u32::MAX,
                "decode_reduce(0) should give the zero element"
            );
        }

        /// Addition: component-wise `(x0 + y0, x1 + y1) mod p`.
        #[test]
        fn fp2_test_add() {
            let zp = fp2_modulus();

            // Random pairs.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i));
                let b = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i + 1));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let zb0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b.x0.encode());
                let zb1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b.x1.encode());
                let (zc0, zc1) = fp2_decode_components(&(a + b).encode());
                assert_eq!(zc0, (&za0 + &zb0) % &zp, "iter {i}: add x0 failed");
                assert_eq!(zc1, (&za1 + &zb1) % &zp, "iter {i}: add x1 failed");
            }

            // a + 0 == a.
            let a = <$Fp2>::decode_reduce(&fp2_test_vector(0));
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!((a + z).equals(&a), u32::MAX, "a + 0 should equal a");

            // 0 + 0 == 0.
            assert_eq!((z + z).is_zero(), u32::MAX, "0 + 0 should be zero");
        }

        /// Subtraction: component-wise `(x0 - y0, x1 - y1) mod p`.
        #[test]
        fn fp2_test_sub() {
            let zp = fp2_modulus();

            // Random pairs.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i));
                let b = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i + 1));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let zb0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b.x0.encode());
                let zb1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b.x1.encode());
                let (zc0, zc1) = fp2_decode_components(&(a - b).encode());
                assert_eq!(zc0, (&zp + &za0 - &zb0) % &zp, "iter {i}: sub x0 failed");
                assert_eq!(zc1, (&zp + &za1 - &zb1) % &zp, "iter {i}: sub x1 failed");
            }

            // a - 0 == a.
            let a = <$Fp2>::decode_reduce(&fp2_test_vector(0));
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!((a - z).equals(&a), u32::MAX, "a - 0 should equal a");

            // a - a == 0.
            assert_eq!((a - a).is_zero(), u32::MAX, "a - a should be zero");

            // 0 - 0 == 0.
            assert_eq!((z - z).is_zero(), u32::MAX, "0 - 0 should be zero");
        }

        /// Negation: `-a` component-wise, and double-negation identity.
        #[test]
        fn fp2_test_neg() {
            let zp = fp2_modulus();

            // Random elements.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let c = -a;
                let (zc0, zc1) = fp2_decode_components(&c.encode());
                assert_eq!(zc0, (&zp - &za0) % &zp, "iter {i}: neg x0 failed");
                assert_eq!(zc1, (&zp - &za1) % &zp, "iter {i}: neg x1 failed");
                assert_eq!((-c).equals(&a), u32::MAX, "iter {i}: -(-a) != a");
            }

            // -0 == 0.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!((-z).is_zero(), u32::MAX, "-0 should be zero");
        }

        /// Multiplication: all three implementations must produce the same result.
        #[test]
        fn fp2_test_mul() {
            let zp = fp2_modulus();

            // Random pairs.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i));
                let b = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i + 1));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let zb0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b.x0.encode());
                let zb1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b.x1.encode());
                let zd0 = (&zp + ((&za0 * &zb0) % &zp) - ((&za1 * &zb1) % &zp)) % &zp;
                let zd1 = ((&za0 * &zb1) + (&za1 * &zb0)) % &zp;

                let (zc0, zc1) = fp2_decode_components(&(a * b).encode());
                assert_eq!(zc0, zd0, "iter {i}: mul (*) x0 failed");
                assert_eq!(zc1, zd1, "iter {i}: mul (*) x1 failed");

                let (zc0, zc1) = fp2_decode_components(&a.mul_schoolbook(&b).encode());
                assert_eq!(zc0, zd0, "iter {i}: mul_schoolbook x0 failed");
                assert_eq!(zc1, zd1, "iter {i}: mul_schoolbook x1 failed");
            }

            // a * 0 == 0 and 0 * 0 == 0.
            let a = <$Fp2>::decode_reduce(&fp2_test_vector(0));
            let z = <$Fp2>::ZERO;
            assert_eq!((a * z).is_zero(), u32::MAX, "a * 0 should be zero");
            assert_eq!((z * z).is_zero(), u32::MAX, "0 * 0 should be zero");
        }

        /// Squaring: `square()` matches `a * a` component-wise.
        #[test]
        fn fp2_test_square() {
            let zp = fp2_modulus();

            // Random elements.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let zd0 = (&zp + ((&za0 * &za0) % &zp) - ((&za1 * &za1) % &zp)) % &zp;
                let zd1 = ((&za0 * &za1) + (&za1 * &za0)) % &zp;
                let (zc0, zc1) = fp2_decode_components(&a.square().encode());
                assert_eq!(zc0, zd0, "iter {i}: square x0 failed");
                assert_eq!(zc1, zd1, "iter {i}: square x1 failed");
                assert_eq!(
                    a.square().equals(&(a * a)),
                    u32::MAX,
                    "iter {i}: square() != a*a"
                );
            }

            // 0^2 == 0.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!(z.square().is_zero(), u32::MAX, "0^2 should be zero");
        }

        /// N-squaring: `n_square(n)` matches repeated `square()` n times.
        #[test]
        fn fp2_test_n_square() {
            // Random elements.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                for n in [1u32, 2, 5, 10, 20] {
                    let mut b = a;
                    for _ in 0..n {
                        b = b.square();
                    }
                    let c = a.n_square(n);
                    assert_eq!(c.equals(&b), u32::MAX, "iter {i}: n_square(n) failed");
                }
            }
            // 0^(2^n) == 0.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            for n in [1u32, 2, 5] {
                let c = z.n_square(n);
                assert_eq!(c.is_zero(), u32::MAX, "n={n}: 0^(2^n) should be zero");
            }
        }

        /// Division / inversion: round-trip and zero-divisor handling.
        #[test]
        fn fp2_test_div_and_invert() {
            // Random pairs (all non-zero since fp2_test_vector is pseudorandom).
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i));
                let b = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i + 1));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let (zc0, zc1) = fp2_decode_components(&(a / b * b).encode());
                assert_eq!(zc0, za0, "iter {i}: division round-trip x0 failed");
                assert_eq!(zc1, za1, "iter {i}: division round-trip x1 failed");
                assert_eq!(
                    (b.invert() * b).equals(&<$Fp2>::ONE),
                    u32::MAX,
                    "iter {i}: invert(b) * b != 1"
                );
            }

            // Division by zero: a / 0 == 0, 0 / 0 == 0, invert(0) == 0.
            let a = <$Fp2>::decode_reduce(&fp2_test_vector(0));
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!((a / z).is_zero(), u32::MAX, "a / 0 should be zero");
            assert_eq!((z / z).is_zero(), u32::MAX, "0 / 0 should be zero");
            assert_eq!(z.invert().is_zero(), u32::MAX, "invert(0) should be zero");

            // Zero numerator: 0 / a == 0 for non-zero a.
            assert_eq!((z / a).is_zero(), u32::MAX, "0 / a should be zero");
        }

        /// Conjugate: `conjugate(a0 + i*a1) == a0 - i*a1`, and double-conjugate identity.
        #[test]
        fn fp2_test_conjugate() {
            let zp = fp2_modulus();

            // Random elements.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                let c = a.conjugate();
                let (zc0, zc1) = fp2_decode_components(&c.encode());
                assert_eq!(zc0, za0, "iter {i}: conjugate changed x0");
                assert_eq!(zc1, (&zp - &za1) % &zp, "iter {i}: conjugate x1 wrong");
                assert_eq!(
                    c.conjugate().equals(&a),
                    u32::MAX,
                    "iter {i}: double conjugate != a"
                );
                let mut d = a;
                d.set_conjugate();
                assert_eq!(
                    d.equals(&c),
                    u32::MAX,
                    "iter {i}: set_conjugate != conjugate"
                );
            }

            // conjugate(0) == 0.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!(
                z.conjugate().is_zero(),
                u32::MAX,
                "conjugate(0) should be zero"
            );
        }

        /// ZETA constant: `i^2 == -1` and `MINUS_ZETA == -ZETA`.
        #[test]
        fn fp2_test_zeta_constants() {
            let zeta = <$Fp2>::ZETA;
            assert_eq!(
                zeta.square().equals(&<$Fp2>::MINUS_ONE),
                u32::MAX,
                "ZETA^2 should be MINUS_ONE"
            );
            assert_eq!(
                (-zeta).equals(&<$Fp2>::MINUS_ZETA),
                u32::MAX,
                "-ZETA should be MINUS_ZETA"
            );
            assert_eq!(
                (zeta + <$Fp2>::MINUS_ZETA).is_zero(),
                u32::MAX,
                "ZETA + MINUS_ZETA should be 0"
            );
        }

        /// `from_i32_pair`, `from_u32_pair`, `from_i64_pair`, `from_u64_pair`.
        #[test]
        fn fp2_test_from_pair_constructors() {
            let zp = fp2_modulus();

            // Random byte sources used as raw integer inputs.
            for i in 0..100 {
                let va = fp2_test_vector(i);

                let x0_i32 = i32::from_le_bytes(va[0..4].try_into().unwrap());
                let x1_i32 = i32::from_le_bytes(va[4..8].try_into().unwrap());
                let (zc0, zc1) =
                    fp2_decode_components(&<$Fp2>::from_i32_pair(x0_i32, x1_i32).encode());
                assert_eq!(
                    zc0,
                    (x0_i32.to_bigint().unwrap() + &zp) % &zp,
                    "iter {i}: from_i32_pair x0"
                );
                assert_eq!(
                    zc1,
                    (x1_i32.to_bigint().unwrap() + &zp) % &zp,
                    "iter {i}: from_i32_pair x1"
                );

                let x0_u32 = x0_i32 as u32;
                let x1_u32 = x1_i32 as u32;
                let (zc0, zc1) =
                    fp2_decode_components(&<$Fp2>::from_u32_pair(x0_u32, x1_u32).encode());
                assert_eq!(
                    zc0,
                    x0_u32.to_bigint().unwrap(),
                    "iter {i}: from_u32_pair x0"
                );
                assert_eq!(
                    zc1,
                    x1_u32.to_bigint().unwrap(),
                    "iter {i}: from_u32_pair x1"
                );

                let x0_i64 = i64::from_le_bytes(va[0..8].try_into().unwrap());
                let x1_i64 = i64::from_le_bytes(va[8..16].try_into().unwrap());
                let (zc0, zc1) =
                    fp2_decode_components(&<$Fp2>::from_i64_pair(x0_i64, x1_i64).encode());
                assert_eq!(
                    zc0,
                    (x0_i64.to_bigint().unwrap() + &zp) % &zp,
                    "iter {i}: from_i64_pair x0"
                );
                assert_eq!(
                    zc1,
                    (x1_i64.to_bigint().unwrap() + &zp) % &zp,
                    "iter {i}: from_i64_pair x1"
                );

                let x0_u64 = x0_i64 as u64;
                let x1_u64 = x1_i64 as u64;
                let (zc0, zc1) =
                    fp2_decode_components(&<$Fp2>::from_u64_pair(x0_u64, x1_u64).encode());
                assert_eq!(
                    zc0,
                    x0_u64.to_bigint().unwrap(),
                    "iter {i}: from_u64_pair x0"
                );
                assert_eq!(
                    zc1,
                    x1_u64.to_bigint().unwrap(),
                    "iter {i}: from_u64_pair x1"
                );
            }

            // Zero inputs must give the zero element.
            assert_eq!(
                <$Fp2>::from_i32_pair(0, 0).is_zero(),
                u32::MAX,
                "from_i32_pair(0,0) should be zero"
            );
            assert_eq!(
                <$Fp2>::from_u32_pair(0, 0).is_zero(),
                u32::MAX,
                "from_u32_pair(0,0) should be zero"
            );
            assert_eq!(
                <$Fp2>::from_i64_pair(0, 0).is_zero(),
                u32::MAX,
                "from_i64_pair(0,0) should be zero"
            );
            assert_eq!(
                <$Fp2>::from_u64_pair(0, 0).is_zero(),
                u32::MAX,
                "from_u64_pair(0,0) should be zero"
            );
        }

        /// `set_x0_small` / `set_x1_small`: only the targeted component changes.
        #[test]
        fn fp2_test_set_component_small() {
            let zp = fp2_modulus();

            // Random elements with a random scalar k sourced from a second vector.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i));
                let vb = fp2_test_vector(2 * i + 1);
                let k = i32::from_le_bytes(vb[0..4].try_into().unwrap());
                let zk = (k.to_bigint().unwrap() + &zp) % &zp;

                // set_x0_small(k): x0 becomes k mod p, x1 is unchanged.
                let mut c = a;
                c.set_x0_small(k);
                let (zc0, zc1) = fp2_decode_components(&c.encode());
                let za1 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x1.encode());
                assert_eq!(zc0, zk, "iter {i}: set_x0_small x0 wrong");
                assert_eq!(zc1, za1, "iter {i}: set_x0_small changed x1");

                // set_x1_small(k): x1 becomes k mod p, x0 is unchanged.
                let mut c = a;
                c.set_x1_small(k);
                let (zc0, zc1) = fp2_decode_components(&c.encode());
                let za0 =
                    ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a.x0.encode());
                assert_eq!(zc0, za0, "iter {i}: set_x1_small changed x0");
                assert_eq!(zc1, zk, "iter {i}: set_x1_small x1 wrong");
            }

            // Zero scalar: set_x0_small(0) must zero x0; set_x1_small(0) must zero x1.
            let a = <$Fp2>::decode_reduce(&fp2_test_vector(0));
            let mut c = a;
            c.set_x0_small(0);
            assert_eq!(c.x0.is_zero(), u32::MAX, "set_x0_small(0) should zero x0");
            let mut c = a;
            c.set_x1_small(0);
            assert_eq!(c.x1.is_zero(), u32::MAX, "set_x1_small(0) should zero x1");
        }

        /// Legendre symbol and `is_square`.
        #[test]
        fn fp2_test_legendre_and_is_square() {
            let nqr = fp2_nqr();

            // Random elements: a^2 is always a QR, nqr * a^2 is always a non-QR
            // (for non-zero a).
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                let e = a * a;
                if a.is_zero() == u32::MAX {
                    assert_eq!(e.legendre(), 0, "iter {i}: legendre(0) should be 0");
                    assert_eq!(
                        e.is_square(),
                        u32::MAX,
                        "iter {i}: is_square(0) should be true"
                    );
                } else {
                    assert_eq!(e.legendre(), 1, "iter {i}: legendre(a^2) should be 1");
                    assert_eq!(
                        e.is_square(),
                        u32::MAX,
                        "iter {i}: is_square(a^2) should be true"
                    );
                    assert_eq!(
                        (e * nqr).legendre(),
                        -1,
                        "iter {i}: legendre(nqr*a^2) should be -1"
                    );
                    assert_eq!(
                        (e * nqr).is_square(),
                        0,
                        "iter {i}: is_square(nqr*a^2) should be false"
                    );
                }
            }

            // Zero: legendre(0) == 0 and zero is considered a square.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!(z.legendre(), 0, "legendre(0) should be 0");
            assert_eq!(z.is_square(), u32::MAX, "is_square(0) should be true");
        }

        /// `is_square_base_field`: correct for pure-real elements.
        #[test]
        fn fp2_test_is_square_base_field() {
            // x0^2 embedded as a real Fp2 element is a square;
            // -(x0^2) embedded as a real element is not.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                if a.x0.is_zero() == 0 {
                    let mut f = <$Fp2>::ZERO;
                    f.x0 = a.x0.square();
                    assert_eq!(
                        f.is_square_base_field(),
                        u32::MAX,
                        "iter {i}: is_square_base_field(x0^2) should be true"
                    );
                    f.x0.set_neg();
                    assert_eq!(
                        f.is_square_base_field(),
                        0,
                        "iter {i}: is_square_base_field(-x0^2) should be false"
                    );
                }
            }

            // Zero real part: is_square_base_field(0) must be true.
            assert_eq!(
                <$Fp2>::ZERO.is_square_base_field(),
                u32::MAX,
                "is_square_base_field(0) should be true"
            );
        }

        /// Square root: success, canonical LSB convention, failure on non-residues.
        #[test]
        fn fp2_test_sqrt() {
            let nqr = fp2_nqr();

            // Random elements: sqrt(a^2) succeeds; sqrt(nqr * a^2) fails.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                let e = a * a;

                // sqrt(a^2) must succeed and verify (sqrt)^2 == a^2.
                let (c, r) = e.sqrt();
                assert_eq!(r, u32::MAX, "iter {i}: sqrt of square failed");
                assert_eq!((c * c).equals(&e), u32::MAX, "iter {i}: sqrt(a^2)^2 != a^2");

                // Canonical root: x0's LSB is 0; when x0 == 0, x1's LSB is also 0.
                let (zc0, zc1) = fp2_decode_components(&c.encode());

                // nqr * a^2 is not a square for non-zero a; sqrt must fail and return zero.
                if a.is_zero() == 0 {
                    let (c, r) = (e * nqr).sqrt();
                    assert_eq!(r, 0x00000000, "iter {i}: sqrt of non-square should fail");
                    assert_eq!(
                        c.is_zero(),
                        u32::MAX,
                        "iter {i}: failed sqrt should return zero"
                    );
                }
            }

            // sqrt(0) == 0: zero is its own square root.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            let (c, r) = z.sqrt();
            assert_eq!(r, u32::MAX, "sqrt(0) should succeed");
            assert_eq!(c.is_zero(), u32::MAX, "sqrt(0) should return zero");
        }

        /// Square root on pure-real elements: both `x0` and `-x0` always have roots in Fp2.
        #[test]
        fn fp2_test_sqrt_real_elements() {
            // For any base-field element x0, both x0 and -x0 are squares in
            // Fp2 because -1 = i^2 is a square in Fp2.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                if a.x0.is_zero() == 0 {
                    let mut f = <$Fp2>::ZERO;
                    f.x0 = a.x0;

                    let (c, r) = f.sqrt();
                    assert_eq!(r, u32::MAX, "iter {i}: sqrt of real element failed");
                    assert_eq!((c * c).equals(&f), u32::MAX, "iter {i}: sqrt(f)^2 != f");

                    let (c, r) = (-f).sqrt();
                    assert_eq!(r, u32::MAX, "iter {i}: sqrt of -real element failed");
                    assert_eq!(
                        (c * c).equals(&(-f)),
                        u32::MAX,
                        "iter {i}: sqrt(-f)^2 != -f"
                    );
                }
            }
        }

        /// Fourth root: success and failure cases.
        #[test]
        fn fp2_test_fourth_root() {
            let nqr = fp2_nqr();

            // Random elements: fourth_root(a^4) succeeds; fourth_root(nqr * a^2) fails.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(i));
                let e = a * a * a * a;

                // fourth_root(a^4) must succeed and verify (root)^4 == a^4.
                let (c, r) = e.fourth_root();
                assert_eq!(r, u32::MAX, "iter {i}: fourth_root of fourth-power failed");
                assert_eq!(
                    (c * c * c * c).equals(&e),
                    u32::MAX,
                    "iter {i}: fourth_root(a^4)^4 != a^4"
                );

                // nqr * a^2 is not a fourth power for non-zero a; must fail.
                if a.is_zero() == 0 {
                    let (_, r) = (a * a * nqr).fourth_root();
                    assert_eq!(
                        r, 0x00000000,
                        "iter {i}: fourth_root of non-fourth-power should fail"
                    );
                }
            }

            // fourth_root(0) == 0: zero is its own fourth root.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            let (c, r) = z.fourth_root();
            assert_eq!(r, u32::MAX, "fourth_root(0) should succeed");
            assert_eq!(c.is_zero(), u32::MAX, "fourth_root(0) should return zero");
        }

        /// `precompute_dlp_tables` + `solve_dlp_2e`.
        #[test]
        fn fp2_test_solve_dlp_2e() {
            // ZETA has order exactly 4 = 2^2, giving us concrete ground truth
            // for all four powers without needing any additional computation.
            let g = <$Fp2>::ZETA;
            let e: usize = 2;

            let (table_idx, table_g, ok) = g.precompute_dlp_tables(e);
            assert_eq!(
                ok,
                u32::MAX,
                "precompute_dlp_tables should succeed for ZETA"
            );

            // The last precomputed power must be g^(2^(e-1)) = g^2 = -1.
            assert_eq!(
                table_g.last().unwrap().equals(&<$Fp2>::MINUS_ONE),
                u32::MAX,
                "last precomputed power should be -1"
            );

            // Check all four powers of ZETA: g^0=1, g^1=ZETA, g^2=-1, g^3=-ZETA.
            let powers: &[($Fp2, u64)] = &[
                (<$Fp2>::ONE, 0),
                (<$Fp2>::ZETA, 1),
                (<$Fp2>::MINUS_ONE, 2),
                (<$Fp2>::MINUS_ZETA, 3),
            ];
            for &(x, expected_v) in powers {
                let (v_bytes, ok) = g.solve_dlp_2e(&x, e, Some((&table_idx, &table_g)));
                assert_eq!(
                    ok,
                    u32::MAX,
                    "solve_dlp_2e should succeed for power {expected_v}"
                );
                // v_bytes is ceil(e/8) bytes — zero-pad to 8 bytes before interpreting as u64.
                let mut buf = [0u8; 8];
                buf[..v_bytes.len()].copy_from_slice(&v_bytes);
                let v = u64::from_le_bytes(buf) % (1u64 << e);
                assert_eq!(
                    v, expected_v,
                    "solve_dlp_2e wrong exponent for power {expected_v}"
                );
            }

            // Failure: ZETA is not a power of MINUS_ONE (order 2 < order of ZETA),
            // so the solve must return failure.
            let (_, ok) = <$Fp2>::MINUS_ONE.solve_dlp_2e(&<$Fp2>::ZETA, e, None);
            assert_eq!(
                ok, 0x00000000,
                "solve_dlp_2e should fail when x is not in <g>"
            );
        }

        /// `is_zero` and `equals` sanity checks.
        #[test]
        fn fp2_test_is_zero_and_equals() {
            // Constant zero and one.
            assert_eq!(
                <$Fp2>::ZERO.is_zero(),
                u32::MAX,
                "ZERO.is_zero() should be true"
            );
            assert_eq!(<$Fp2>::ONE.is_zero(), 0, "ONE.is_zero() should be false");
            assert_eq!(
                <$Fp2>::ZERO.equals(&<$Fp2>::ZERO),
                u32::MAX,
                "ZERO should equal ZERO"
            );
            assert_eq!(
                <$Fp2>::ZERO.equals(&<$Fp2>::ONE),
                0,
                "ZERO should not equal ONE"
            );

            // Random elements: self-equality, additive inverse, commutativity.
            for i in 0..100 {
                let a = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i));
                let b = <$Fp2>::decode_reduce(&fp2_test_vector(2 * i + 1));
                assert_eq!(a.equals(&a), u32::MAX, "iter {i}: a should equal itself");
                assert_eq!(
                    (a + (-a)).is_zero(),
                    u32::MAX,
                    "iter {i}: a + (-a) should be zero"
                );
                assert_eq!(
                    (a + b).equals(&(b + a)),
                    u32::MAX,
                    "iter {i}: addition should be commutative"
                );
            }

            // decode_reduce of zero vector must equal the ZERO constant.
            let z = <$Fp2>::decode_reduce(&fp2_zero_vector());
            assert_eq!(
                z.is_zero(),
                u32::MAX,
                "decode_reduce(zero vector) should be zero"
            );
            assert_eq!(
                z.equals(&<$Fp2>::ZERO),
                u32::MAX,
                "decoded zero should equal ZERO constant"
            );
        }

        #[test]
        fn test_fp2_xi_decomposition() {
            let y = <$Fp2>::from_u32_pair(123, 345);
            let (y0, y1) = y.xi();
            assert_eq!(y.x0.equals(&y0), u32::MAX);
            assert_eq!(y.x1.equals(&y1), u32::MAX);
            assert_eq!(y.x0.equals(&y.x0()), u32::MAX);
            assert_eq!(y.x1.equals(&y.x1()), u32::MAX);
        }

        /// Predefined constants: arithmetic identities.
        #[test]
        fn fp2_test_constants() {
            let one = <$Fp2>::ONE;
            let two = <$Fp2>::TWO;
            let three = <$Fp2>::THREE;
            let four = <$Fp2>::FOUR;
            let minus_one = <$Fp2>::MINUS_ONE;

            assert_ne!(<$Fp2>::ZERO.is_zero(), 0, "ZERO.is_zero()");
            assert_ne!((one + minus_one).is_zero(), 0, "ONE + MINUS_ONE == 0");
            assert_ne!((one + one).equals(&two), 0, "1+1 == TWO");
            assert_ne!((two + one).equals(&three), 0, "2+1 == THREE");
            assert_ne!((three + one).equals(&four), 0, "3+1 == FOUR");
            assert_ne!((minus_one * minus_one).equals(&one), 0, "(-1)*(-1) == 1");
        }

        #[test]
        fn test_fp2_trait_static_methods() {
            use fp2::traits::Fp2;
            fn via_trait<F: Fp2>(x: F, y: F) {
                // Static methods
                let _ = F::from_i32_pair(1, 2);
                let _ = F::from_u32_pair(1, 2);
                let _ = F::from_i64_pair(1, 2);
                let _ = F::from_u64_pair(1, 2);

                // Constants
                assert_eq!((F::ZETA + F::MINUS_ZETA).is_zero(), u32::MAX);
                assert_eq!(F::ZETA.square().equals(&F::MINUS_ONE), u32::MAX);

                // Instance methods unique to Fp2 trait
                let _ = x.conjugate();
                let _ = x.is_square_base_field();

                // invert via trait dispatch (the bug we just fixed in Fp)
                let inv = x.invert();
                assert_eq!((inv * x).equals(&F::ONE), u32::MAX);

                // conjugate is its own inverse
                assert_eq!(x.conjugate().conjugate().equals(&x), u32::MAX);

                // set_conjugate matches conjugate
                let mut c = x;
                c.set_conjugate();
                assert_eq!(c.equals(&x.conjugate()), u32::MAX);

                // sqrt round-trip
                let (root, ok) = x.square().sqrt();
                assert_eq!(ok, u32::MAX);
                assert_eq!(root.square().equals(&x.square()), u32::MAX);

                // fourth_root round-trip
                let x4 = x.square().square();
                let (root, ok) = x4.fourth_root();
                assert_eq!(ok, u32::MAX);
                assert_eq!(root.square().square().equals(&x4), u32::MAX);

                // set_x0_small / set_x1_small
                let mut c = x;
                c.set_x0_small(1);
                c.set_x1_small(0);
                assert_eq!(c.equals(&F::ONE), u32::MAX);
            }
            via_trait(<$Fp2>::from_u32_pair(3, 7), <$Fp2>::from_u32_pair(5, 11));
        }
    };
} // End of macro: define_fp2_tests
