#[cfg(feature = "test_macros")]
#[cfg(test)]
mod tests {
    // Random prime with no nice properties for Montgomery friendliness
    mod fp_ugly_tests {
        // Field modulus
        static MODULUS: [u64; 2] = [0x5A0E852097C48043, 0x7EA2A3A646684E9D];

        // Fp139: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 2^127 - 1
        fp2::define_fp_core!(typename = FpUgly, modulus = MODULUS,);
        fp2::define_fp_tests!(FpUgly);

        // FpUglyExt: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_from_modulus!(typename = FpUglyExt, base_typename = Fp, modulus = MODULUS,);
        fp2::define_fp2_tests!(FpUglyExt, MODULUS, 1);
    }

    mod fp127_tests {
        // Field modulus
        static MODULUS: [u64; 2] = [0xFFFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF];

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
    }

    mod fp251_tests {
        // Field modulus
        static MODULUS: [u64; 4] = [
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
    }

    mod fp383_tests {
        // Field modulus
        static MODULUS: [u64; 6] = [
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
    }
}
