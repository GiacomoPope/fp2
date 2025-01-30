#[cfg_attr(feature = "test_macros", macro_export)]
#[cfg_attr(not(feature = "test_macros"), allow(unused_macros))]
macro_rules! define_fp_tests {
    ($Fp:ty) => {
        use ::num_bigint::ToBigInt as _;
        use ::sha2::Digest as _;

        fn check_fp_ops(va: &[u8], vb: &[u8], with_sqrt_and_fourth_root: bool) {
            let mut zpww = [0u32; <$Fp>::N * 2];
            for i in 0..<$Fp>::N {
                zpww[2 * i] = <$Fp>::MODULUS[i] as u32;
                zpww[2 * i + 1] = (<$Fp>::MODULUS[i] >> 32) as u32;
            }
            println!("Hello {}", <$Fp>::BIT_LENGTH);

            let zp = ::num_bigint::BigInt::from_slice(::num_bigint::Sign::Plus, &zpww);
            let zpz = &zp << 64;

            let a = <$Fp>::decode_reduce(va);
            let b = <$Fp>::decode_reduce(vb);
            let za = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, va);
            let zb = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, vb);

            let vc = a.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = &za % &zp;
            assert!(zc == zd);

            let c = a + b;
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&za + &zb) % &zp;
            assert!(zc == zd);

            let c = a - b;
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = ((&zpz + &za) - (&zb % &zp)) % &zp;
            assert!(zc == zd);

            let c = -a;
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&zp - (&za % &zp)) % &zp;
            assert!(zc == zd);

            let c = a * b;
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&za * &zb) % &zp;
            assert!(zc == zd);

            let c = a.square();
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&za * &za) % &zp;
            assert!(zc == zd);

            let mut c = a.half();
            c += c;
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = &za % &zp;
            assert!(zc == zd);

            let c = a.mul2();
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&za + &za) % &zp;
            assert!(zc == zd);

            let c = a.mul3();
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&za + &za + &za) % &zp;
            assert!(zc == zd);

            let c = a.mul4();
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (&za + &za + &za + &za) % &zp;
            assert!(zc == zd);

            let k = u32::from_le_bytes(*<&[u8; 4]>::try_from(&vb[0..4]).unwrap()) as i32;
            let c = a.mul_small(k);
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = ((&za % &zp) * k + &zpz) % &zp;
            assert!(zc == zd);

            let c = <$Fp>::from_i32(k);
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (k.to_bigint().unwrap() + &zp) % &zp;
            assert!(zc == zd);

            let k = k as u32;
            let c = <$Fp>::from_u32(k);
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = k.to_bigint().unwrap();
            assert!(zc == zd);

            let k = u64::from_le_bytes(*<&[u8; 8]>::try_from(&vb[0..8]).unwrap()) as i64;
            let c = <$Fp>::from_i64(k);
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = (k.to_bigint().unwrap() + &zp) % &zp;
            assert!(zc == zd);

            let k = k as u64;
            let c = <$Fp>::from_u64(k);
            let vc = c.encode();
            let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
            let zd = k.to_bigint().unwrap();
            assert!(zc == zd);

            let c = a / b;
            if b.iszero() != 0 {
                assert!(c.iszero() != 0);
            } else {
                let c = c * b;
                let vc = c.encode();
                let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
                let zd = &za % &zp;
                assert!(zc == zd);
            }

            let c = a.square();
            if c.iszero() != 0 {
                assert!(c.legendre() == 0);
            } else {
                assert!(c.legendre() == 1);
                let c = -c;
                assert!(c.legendre() == -1);
            }

            // Different sums of products
            let c0 = a * b + b * b;
            let c1 = <$Fp>::sum_of_products(&a, &b, &b, &b);
            assert!(c1.equals(&c0) != 0);

            // Difference of products
            let c = a * b;
            let d = b * c;
            let c0 = a * b - c * d;
            let c1 = <$Fp>::difference_of_products(&a, &b, &c, &d);
            let c2 = <$Fp>::sum_of_products(&a, &b, &c, &(-&d));
            assert!(c1.equals(&c0) != 0);
            assert!(c2.equals(&c0) != 0);

            if with_sqrt_and_fourth_root {
                let (c, r) = (a * a).sqrt();
                assert!(r == 0xFFFFFFFF);
                let vc = c.encode();
                let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
                assert!(zc.bit(0) == false);
                let zc = (&zc * &zc) % &zp;
                let zd = (&za * &za) % &zp;
                assert!(zc == zd);
                if a.iszero() == 0 {
                    let (c, r) = (-(a * a)).sqrt();
                    assert!(c.iszero() == 0xFFFFFFFF);
                    assert!(r == 0x00000000);
                }

                let (c, r) = (a * a * a * a).fourth_root();
                assert!(r == 0xFFFFFFFF);
                let vc = c.encode();
                let zc = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &vc);
                assert!(zc.bit(0) == false);
                let zc = (&zc * &zc * &zc * &zc) % &zp;
                let zd = (&za * &za * &za * &za) % &zp;
                assert!(zc == zd);
                if a.iszero() == 0 {
                    let (c, r) = (-(a * a * a * a)).sqrt();
                    assert!(c.iszero() == 0xFFFFFFFF);
                    assert!(r == 0x00000000);
                }
            }
        }

        #[test]
        fn fp_ops() {
            let mut va = [0u8; (<$Fp>::ENCODED_LENGTH + 64) & !31usize];
            let mut vb = [0u8; (<$Fp>::ENCODED_LENGTH + 64) & !31usize];
            for i in 0..300 {
                let mut sh = ::sha2::Sha256::new();
                for j in 0..(va.len() >> 5) {
                    sh.update(((256 * i + 8 * j + 0) as u64).to_le_bytes());
                    va[(32 * j)..(32 * j + 32)].copy_from_slice(&sh.finalize_reset());
                }
                for j in 0..(vb.len() >> 5) {
                    sh.update(((256 * i + 8 * j + 1) as u64).to_le_bytes());
                    vb[(32 * j)..(32 * j + 32)].copy_from_slice(&sh.finalize_reset());
                }
                if i == 10 || i == 12 {
                    va.fill(0);
                }
                if i == 11 || i == 12 {
                    vb.fill(0);
                }
                check_fp_ops(&va, &vb, i < 30);
            }
        }
    };
} // End of macro: define_fp_tests

// Macro expectations:
#[cfg_attr(feature = "test_macros", macro_export)]
#[cfg_attr(not(feature = "test_macros"), allow(unused_macros))]
macro_rules! define_fp2_tests {
    ($Fp:ty, $Fp2:ty) => {
        use ::num_bigint::ToBigInt as _;
        use ::sha2::Digest as _;

        fn check_fp2_ops(va: &[u8], vb: &[u8], with_sqrt_and_fourth_root: bool) {
            let mut zpww = [0u32; <$Fp>::N * 2];
            for i in 0..<$Fp>::N {
                zpww[2 * i] = <$Fp>::MODULUS[i] as u32;
                zpww[2 * i + 1] = (<$Fp>::MODULUS[i] >> 32) as u32;
            }
            let zp = ::num_bigint::BigInt::from_slice(::num_bigint::Sign::Plus, &zpww);

            let alen = va.len() >> 1;
            let blen = vb.len() >> 1;

            let a0 = <$Fp>::decode_reduce(&va[..alen]);
            let a1 = <$Fp>::decode_reduce(&va[alen..]);
            let b0 = <$Fp>::decode_reduce(&vb[..blen]);
            let b1 = <$Fp>::decode_reduce(&vb[blen..]);
            let za0 = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a0.encode());
            let za1 = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &a1.encode());
            let zb0 = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b0.encode());
            let zb1 = ::num_bigint::BigInt::from_bytes_le(::num_bigint::Sign::Plus, &b1.encode());
            let a = <$Fp2>::new(&a0, &a1);
            let b = <$Fp2>::new(&b0, &b1);

            let c = a + b;
            let vc = c.encode();
            let zc0 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[..<$Fp>::ENCODED_LENGTH],
            );
            let zc1 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[<$Fp>::ENCODED_LENGTH..],
            );
            let zd0 = (&za0 + &zb0) % &zp;
            let zd1 = (&za1 + &zb1) % &zp;
            assert!(zc0 == zd0 && zc1 == zd1);

            let c = a - b;
            let vc = c.encode();
            let zc0 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[..<$Fp>::ENCODED_LENGTH],
            );
            let zc1 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[<$Fp>::ENCODED_LENGTH..],
            );
            let zd0 = (&zp + &za0 - &zb0) % &zp;
            let zd1 = (&zp + &za1 - &zb1) % &zp;
            assert!(zc0 == zd0 && zc1 == zd1);

            let c = a * b;
            let vc = c.encode();
            let zc0 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[..<$Fp>::ENCODED_LENGTH],
            );
            let zc1 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[<$Fp>::ENCODED_LENGTH..],
            );
            let zd0 = (&zp + ((&za0 * &zb0) % &zp) - ((&za1 * &zb1) % &zp)) % &zp;
            let zd1 = ((&za0 * &zb1) + (&za1 * &zb0)) % &zp;
            assert!(zc0 == zd0 && zc1 == zd1);

            let c = a.mul_new(b);
            let vc = c.encode();
            let zc0 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[..<$Fp>::ENCODED_LENGTH],
            );
            let zc1 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[<$Fp>::ENCODED_LENGTH..],
            );
            let zd0 = (&zp + ((&za0 * &zb0) % &zp) - ((&za1 * &zb1) % &zp)) % &zp;
            let zd1 = ((&za0 * &zb1) + (&za1 * &zb0)) % &zp;
            assert!(zc0 == zd0 && zc1 == zd1);

            let c = a.square();
            let vc = c.encode();
            let zc0 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[..<$Fp>::ENCODED_LENGTH],
            );
            let zc1 = ::num_bigint::BigInt::from_bytes_le(
                ::num_bigint::Sign::Plus,
                &vc[<$Fp>::ENCODED_LENGTH..],
            );
            let zd0 = (&zp + ((&za0 * &za0) % &zp) - ((&za1 * &za1) % &zp)) % &zp;
            let zd1 = ((&za0 * &za1) + (&za1 * &za0)) % &zp;
            assert!(zc0 == zd0 && zc1 == zd1);

            let c = a / b;
            if b.iszero() != 0 {
                assert!(c.iszero() == 0xFFFFFFFF);
            } else {
                let c = c * b;
                let vc = c.encode();
                let zc0 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &vc[..<$Fp>::ENCODED_LENGTH],
                );
                let zc1 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &vc[<$Fp>::ENCODED_LENGTH..],
                );
                assert!(zc0 == za0 && zc1 == za1);
            }

            let c = b.invert();
            if b.iszero() != 0 {
                assert!(c.iszero() == 0xFFFFFFFF);
            } else {
                let c = c * b;
                assert!(c.equals(&<$Fp2>::ONE) == 0xFFFFFFFF);
            }

            if with_sqrt_and_fourth_root {
                let e = a * a;
                let (c, r) = e.sqrt();
                assert!(r == 0xFFFFFFFF);
                assert!((c * c).equals(&e) == 0xFFFFFFFF);
                let vc = c.encode();
                let zc0 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &vc[..<$Fp>::ENCODED_LENGTH],
                );
                let zc1 = ::num_bigint::BigInt::from_bytes_le(
                    ::num_bigint::Sign::Plus,
                    &vc[<$Fp>::ENCODED_LENGTH..],
                );
                assert!(zc0.bit(0) == false);
                if zc0.sign() == ::num_bigint::Sign::NoSign {
                    assert!(zc1.bit(0) == false);
                }
                if a.iszero() == 0 {
                    assert!(e.legendre() == 1);
                    let e = a * a * <$Fp2>::NQR;
                    assert!(e.legendre() == -1);
                    let (c, r) = e.sqrt();
                    assert!(r == 0);
                    assert!(c.iszero() == 0xFFFFFFFF);
                    let (_, r) = e.fourth_root();
                    assert!(r == 0);
                } else {
                    assert!(e.legendre() == 0);
                }

                if a0.iszero() == 0 {
                    let f = <$Fp2>::new(&a0, &<$Fp>::ZERO);
                    let (c, r) = f.sqrt();
                    assert!(r == 0xFFFFFFFF);
                    assert!((c * c).equals(&f) == 0xFFFFFFFF);
                    let g = -f;
                    let (c, r) = g.sqrt();
                    assert!(r == 0xFFFFFFFF);
                    assert!((c * c).equals(&g) == 0xFFFFFFFF);
                }

                let e = a * a * a * a;
                let (c, r) = e.fourth_root();

                assert!(r == 0xFFFFFFFF);
                assert!((c * c * c * c).equals(&e) == 0xFFFFFFFF);
            }
        }

        #[test]
        fn fp2_ops() {
            let mut va = [0u8; (2 * <$Fp>::ENCODED_LENGTH + 64) & !31usize];
            let mut vb = [0u8; (2 * <$Fp>::ENCODED_LENGTH + 64) & !31usize];
            for i in 0..100 {
                let mut sh = ::sha2::Sha256::new();
                for j in 0..(va.len() >> 5) {
                    sh.update(((16 * i + 8 * j + 0) as u64).to_le_bytes());
                    va[(32 * j)..(32 * j + 32)].copy_from_slice(&sh.finalize_reset());
                }
                for j in 0..(vb.len() >> 5) {
                    sh.update(((16 * i + 8 * j + 1) as u64).to_le_bytes());
                    vb[(32 * j)..(32 * j + 32)].copy_from_slice(&sh.finalize_reset());
                }
                if i == 10 || i == 12 {
                    va.fill(0);
                }
                if i == 11 || i == 12 {
                    vb.fill(0);
                }
                check_fp2_ops(&va, &vb, true);
            }
        }
    };
} // End of macro: define_fp2_tests
