#[cfg(feature = "test_macros")]
#[cfg(test)]
mod tests {
    mod fp139_tests {
        // Fp139: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 2^74 * 3^41 - 1
        fp2::define_fp_core!(
            typename = Fp139,
            modulus = [
                0xFFFFFFFFFFFFFFFF_u64,
                0xA873D9ED7EE18BFF_u64,
                0x00000000000007E8_u64
            ],
        );

        // Fp139Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_core!(typename = Fp139Ext, base_field = Fp139,);

        fp2::define_fp_tests!(Fp139);
        fp2::define_fp2_tests!(Fp139, Fp139Ext, 10);
    }

    mod fp251_tests {
        // Fp251: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 5*2^248 - 1
        fp2::define_fp_core!(
            typename = Fp251,
            modulus = [
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0x04FFFFFFFFFFFFFFu64
            ],
        );

        // Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_core!(typename = Fp251Ext, base_field = Fp251,);

        fp2::define_fp_tests!(Fp251);
        fp2::define_fp2_tests!(Fp251, Fp251Ext, 5);
    }

    mod fp383_tests {
        // Fp383: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 65 * 2**376 - 1
        fp2::define_fp_core!(
            typename = Fp383,
            modulus = [
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0x40FFFFFFFFFFFFFFu64
            ],
        );

        // Fp383Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2::define_fp2_core!(typename = Fp383Ext, base_field = Fp383,);

        // For define_fp2_tests we must include a u64 nqr_re such that
        // nqr_re + i is a non-quadratic residue in Fp2
        fp2::define_fp_tests!(Fp383);
        fp2::define_fp2_tests!(Fp383, Fp383Ext, 6);
    }
}
