#[cfg(feature = "test_macros")]
#[cfg(test)]
mod tests {
    // Random prime with no nice properties for Montgomery friendliness
    mod fp_ugly_tests {
        // Field modulus
        const MODULUS: [u64; 2] = [0x5A0E852097C48043, 0x7EA2A3A646684E9D];

        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp_core!(typename = FpUgly, modulus = MODULUS,);
        fp2::define_fp_tests!(FpUgly);

        // FpUglyExt: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_from_modulus!(typename = FpUglyExt, base_typename = Fp, modulus = MODULUS,);
        fp2::define_fp2_tests!(FpUglyExt, MODULUS, 1);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(!FpUgly::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }

    mod fp127_tests {
        // Field modulus
        const MODULUS: [u64; 2] = [0xFFFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF];

        // Fp139: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 2^127 - 1
        fp2::define_fp_core!(typename = Fp127, modulus = MODULUS,);
        fp2::define_fp_tests!(Fp127);

        // Fp127Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_from_modulus!(typename = Fp127Ext, base_typename = Fp, modulus = MODULUS,);
        fp2::define_fp2_tests!(Fp127Ext, MODULUS, 2);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(!Fp127::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }

    mod fp251_tests {
        // Field modulus
        const MODULUS: [u64; 4] = [
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0x04FFFFFFFFFFFFFF,
        ];

        // Fp251: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 5*2^248 - 1
        fp2::define_fp_core!(typename = Fp251, modulus = MODULUS,);

        // Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_from_type!(typename = Fp251Ext, base_field = Fp251,);

        fp2::define_fp_tests!(Fp251);
        fp2::define_fp2_tests!(Fp251Ext, MODULUS, 5);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(!Fp251::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }

    mod fp383_tests {
        // Field modulus
        const MODULUS: [u64; 6] = [
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0x40FFFFFFFFFFFFFF,
        ];

        // Fp383: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 65 * 2**376 - 1
        fp2::define_fp_core!(typename = Fp383, modulus = MODULUS,);

        // Fp383Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_from_type!(typename = Fp383Ext, base_field = Fp383,);

        // For define_fp2_tests we must include a u64 nqr_re such that
        // nqr_re + i is a non-quadratic residue in Fp2
        fp2::define_fp_tests!(Fp383);
        fp2::define_fp2_tests!(Fp383Ext, MODULUS, 6);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(!Fp383::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }

    mod fp434_tests {
        // NIST lvl 1 SIKE prime: p = 2^216 * 3^137 - 1
        // Fp434Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        const MODULUS: [u64; 7] = [
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFDC1767AE2FFFFFF,
            0x7BC65C783158AEA3,
            0x6CFC5FD681C52056,
            0x0002341F27177344,
        ];

        fp2::define_fp2_from_modulus!(
            typename = Fp434Ext,
            base_typename = Fp434,
            modulus = MODULUS,
        );

        // For define_fp2_tests we must include a u64 nqr_re such that
        // nqr_re + i is a non-quadratic residue in Fp2
        fp2::define_fp_tests!(Fp434);
        fp2::define_fp2_tests!(Fp434Ext, MODULUS, 2);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(!Fp434::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }

    // This modulus is very close to the value R = 2^(64 * 14) and so
    // needs an extra conditional subtraction in the sum of products
    // method.
    mod fp896_tests {
        const MODULUS: [u64; 14] = [
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xA7FFFFFFFFFFFFFF,
        ];
        // Fp896: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        fp2::define_fp_core!(typename = Fp896, modulus = MODULUS,);

        // Fp896Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        fp2::define_fp2_from_type!(typename = Fp896Ext, base_field = Fp896,);

        fp2::define_fp_tests!(Fp896);
        fp2::define_fp2_tests!(Fp896Ext, MODULUS, 2);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(Fp896::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }

    // When the word length is larger than 20, we slightly modify set_mul
    mod fp1554_tests {
        static MODULUS: [u64; 25] = [
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0x0000000000047FFF,
        ];
        // Fp896: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        fp2::define_fp_core!(typename = Fp1554, modulus = MODULUS,);
        fp2::define_fp_tests!(Fp1554);

        #[test]
        fn check_sum_of_products_flag() {
            assert!(!Fp1554::SUM_OF_PRODUCTS_ADDITIONAL_SUB);
        }
    }
}
