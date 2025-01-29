#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// ========================================================
// Definitions of base fields GF(p) = Z / pZ
// Constants defined are for the macro and generated from
// gen_fp.sage
// ========================================================

// Fp139: a finite field element GF(p) with p = 3 mod 4.
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
// p = 2^74 * 3^41 - 1
crate::finitefield::fp_gen::define_fp_core!(
    type_name = Fp139,
    words = 3_usize,
    bit_len = 139_usize,
    modulus = [0xFFFFFFFFFFFFFFFF, 0xA873D9ED7EE18BFF, 0x00000000000007E8],
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

// Fp251: a finite field element GF(p) with p = 3 mod 4.
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
// p = 5*2^248 - 1
crate::finitefield::fp_gen::define_fp_core!(
    type_name = Fp251,
    words = 4_usize,
    bit_len = 251_usize,
    modulus = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x04FFFFFFFFFFFFFF
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

// ========================================================
// Definitions of extension fields above the base fields
// GF(p^2) with modulus x^2 + 1 = 0 (using p = 3 mod 4)
//
// Macro expectation:
//    - The name of the type
//    - The type of the base GF(p)
//    - The real component of a NQR, such that i + NQR_RE
//      is not a square in GF(p^2)
// ========================================================

// Fp139Ext: a finite field element GF(p^2) with modulus x^2 + 1.
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
crate::finitefield::fp2_gen::define_fp2_core!(
    type_name = Fp139Ext,
    base_field = crate::fields::Fp139,
    nqr_re = [0xB3F48FE34D46EFAA, 0xF314F2E6FA6E0B8E, 0x00000000000003C5]
);

// Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1.
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
crate::finitefield::fp2_gen::define_fp2_core!(
    type_name = Fp251Ext,
    base_field = crate::fields::Fp251,
    nqr_re = [
        0x104AEFD09BF45BF0,
        0xDA5FC9924601A4F5,
        0x9B25B542062AB710,
        0x01B6338B8D91FB50
    ]
);

#[cfg(test)]
mod fp139_tests {
    use crate::fields::Fp139;
    use crate::fields::Fp139Ext;

    crate::finitefield::fp_gen::define_fp_tests!(Fp139);
    crate::finitefield::fp2_gen::define_fp2_tests!(Fp139, Fp139Ext);
}

#[cfg(test)]
mod fp251_tests {
    use crate::fields::Fp251;
    use crate::fields::Fp251Ext;

    crate::finitefield::fp_gen::define_fp_tests!(Fp251);
    crate::finitefield::fp2_gen::define_fp2_tests!(Fp251, Fp251Ext);
}
