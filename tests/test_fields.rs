#[cfg(feature = "test_macros")]
#[cfg(test)]
mod tests {
    mod fp139_tests {
        // Fp139: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 2^74 * 3^41 - 1
        fp2_rs::define_fp_core!(
            typename = Fp139,
            modulus = [
                0xFFFFFFFFFFFFFFFF_u64,
                0xA873D9ED7EE18BFF_u64,
                0x00000000000007E8_u64
            ],
            half_modulus = [0x0000000000000000, 0x5439ECF6BF70C600, 0x00000000000003F4],
            mont_r = [0x00205E71C12F4F15, 0x0C05B368494B8400, 0x00000000000006C0],
            neg_r = [0xFFDFA18E3ED0B0EA, 0x9C6E2685359607FF, 0x0000000000000128],
            two_r = [0x0040BCE3825E9E2B, 0x6F978CE313B57C00, 0x0000000000000597],
            three_r = [0x00611B55438DED41, 0xD329665DDE1F7400, 0x000000000000046E],
            four_r = [0x008179C704BD3C57, 0x36BB3FD8A8896C00, 0x0000000000000346],
            r_sqr = [0xEB062ACE18383AFC, 0x62F4320962D37D74, 0x0000000000000039],
            minus_p_inv = 1_u64,
            div_correction = [0xAFB251EC659DFE3D, 0x5D7F49EE6F1D4D66, 0x000000000000008F],
            reduce_const = [0xC6AE150802914D74, 0xC368D55E3A868DE9, 0x0000000000000742],
            window_len = 5_usize,
            sqrt_el = 14_usize,
            sqrt_eh = [12, 12, 24, 29, 23, 22, 7, 27, 19, 3, 10, 17, 30, 3],
            fourth_root_el = 14_usize,
            fourth_root_eh = [6, 6, 28, 30, 11, 27, 19, 29, 25, 1, 21, 8, 31, 1],
            p1 = 4246015611_u64,
            p1_div_m = 212669779836060700_u64,
        );

        // Fp139Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2_rs::define_fp2_core!(
            typename = Fp139Ext,
            base_field = Fp139,
            nqr_re = [0x0143B0718BD916DA, 0x349A32A6E5E6C800, 0x000000000000043B]
        );

        fp2_rs::define_fp_tests!(Fp139);
        fp2_rs::define_fp2_tests!(Fp139, Fp139Ext);
    }

    mod fp251_tests {

        // Fp251: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        // p = 5*2^248 - 1
        fp2_rs::define_fp_core!(
            typename = Fp251,
            modulus = [
                0xFFFFFFFFFFFFFFFF_u64,
                0xFFFFFFFFFFFFFFFF_u64,
                0xFFFFFFFFFFFFFFFF_u64,
                0x04FFFFFFFFFFFFFF_u64
            ],
            half_modulus = [
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0280000000000000
            ],
            mont_r = [
                0x0000000000000033,
                0x0000000000000000,
                0x0000000000000000,
                0x0100000000000000
            ],
            neg_r = [
                0xFFFFFFFFFFFFFFCC,
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
                0x03FFFFFFFFFFFFFF
            ],
            two_r = [
                0x0000000000000066,
                0x0000000000000000,
                0x0000000000000000,
                0x0200000000000000
            ],
            three_r = [
                0x0000000000000099,
                0x0000000000000000,
                0x0000000000000000,
                0x0300000000000000
            ],
            four_r = [
                0x00000000000000CC,
                0x0000000000000000,
                0x0000000000000000,
                0x0400000000000000
            ],
            r_sqr = [
                0x3333333333333D70,
                0x3333333333333333,
                0x3333333333333333,
                0x0333333333333333
            ],
            minus_p_inv = 1_u64,
            div_correction = [
                0x49BA5E3BCD35A858,
                0xF7CED916872B020C,
                0x72B020C49BA5E353,
                0x025E353F7CED9168
            ],
            reduce_const = [
                0x3333333333333333,
                0x3333333333333333,
                0x3333333333333333,
                0x0100000000000033
            ],
            window_len = 5_usize,
            sqrt_el = 49_usize,
            sqrt_eh = [10],
            fourth_root_el = 49_usize,
            fourth_root_eh = [5],
            p1 = 2684354559_u64,
            p1_div_m = 11068046455220847252_u64,
        );

        // Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2_rs::define_fp2_core!(
            typename = Fp251Ext,
            base_field = Fp251,
            nqr_re = [
                0x0000000000000100_u64,
                0x0000000000000000_u64,
                0x0000000000000000_u64,
                0x0000000000000000_u64
            ]
        );

        fp2_rs::define_fp_tests!(Fp251);
        fp2_rs::define_fp2_tests!(Fp251, Fp251Ext);
    }

    mod fp383_tests {
        // Fp383: a finite field element GF(p) with p = 3 mod 4.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2_rs::define_fp_core!(
            typename = Fp383,
            modulus = [
                0xFFFFFFFFFFFFFFFF_u64,
                0xFFFFFFFFFFFFFFFF_u64,
                0xFFFFFFFFFFFFFFFF_u64,
                0xFFFFFFFFFFFFFFFF_u64,
                0xFFFFFFFFFFFFFFFF_u64,
                0x40FFFFFFFFFFFFFF_u64
            ],
            half_modulus = [
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x2080000000000000
            ],
            mont_r = [
                0x0000000000000003,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x3D00000000000000
            ],
            neg_r = [
                0xFFFFFFFFFFFFFFFC,
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF,
                0x03FFFFFFFFFFFFFF
            ],
            two_r = [
                0x0000000000000007,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x3900000000000000
            ],
            three_r = [
                0x000000000000000B,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x3500000000000000
            ],
            four_r = [
                0x000000000000000F,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x3100000000000000
            ],
            r_sqr = [
                0x3F03F03F03F03F13,
                0x03F03F03F03F03F0,
                0xF03F03F03F03F03F,
                0x3F03F03F03F03F03,
                0x03F03F03F03F03F0,
                0x1D3F03F03F03F03F
            ],
            minus_p_inv = 1_u64,
            div_correction = [
                0xD2220C7F2ED57A49,
                0x135335144B641933,
                0x0C221D3394164B45,
                0x46498DFB36D4EF28,
                0x995A997AD0D06037,
                0x106EE23EE8BBD4A1
            ],
            reduce_const = [
                0x03F03F03F03F03F0,
                0xF03F03F03F03F03F,
                0x3F03F03F03F03F03,
                0x03F03F03F03F03F0,
                0xF03F03F03F03F03F,
                0x1000000000000003
            ],
            window_len = 5_usize,
            sqrt_el = 74_usize,
            sqrt_eh = [16, 0, 1],
            fourth_root_el = 74_usize,
            fourth_root_eh = [8, 16],
            p1 = 2181038079_u64,
            p1_div_m = 17879151965019966409_u64,
        );

        // Fp383Ext: a finite field element GF(p^2) with modulus x^2 + 1.
        // Contents are opaque, all functions are constant-time.
        // Macro input generated with scripts/gen_fp.sage
        fp2_rs::define_fp2_core!(
            typename = Fp383Ext,
            base_field = Fp383,
            nqr_re = [
                0x0000000000000017,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x0000000000000000,
                0x2900000000000000
            ]
        );

        fp2_rs::define_fp_tests!(Fp383);
        fp2_rs::define_fp2_tests!(Fp383, Fp383Ext);
    }
}
