#[cfg(feature = "test_macros")]
#[cfg(test)]
mod tests {
    mod fp139_tests {
        // Field modulus
        pub static MODULUS: [u64; 3] = [0xFFFFFFFFFFFFFFFF, 0xA873D9ED7EE18BFF, 0x00000000000007E8];

        // Fp139: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 2^74 * 3^41 - 1
        fp2::define_fp_core!(typename = Fp139, modulus = MODULUS,);
        fp2::define_fp_tests!(Fp139);

        // Fp139Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_from_modulus!(typename = Fp139Ext, modulus = MODULUS,);
        fp2::define_fp2_tests!(Fp139Ext, MODULUS, 10);
    }

    mod fp251_tests {
        // Field modulus
        pub static MODULUS: [u64; 4] = [
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
        pub static MODULUS: [u64; 6] = [
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
