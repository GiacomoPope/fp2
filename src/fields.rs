#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// ========================================================
// Definitions of base fields GF(p) = Z / pZ
// Constants defined are for the macro and generated from
// gen_fp.sage
// ========================================================

// p = 2^74 * 3^41 - 1
crate::finitefield::fp_gen::define_fp_core!(
    Fp139,
    3_usize,
    139_usize,
    [0xFFFFFFFFFFFFFFFF, 0xA873D9ED7EE18BFF, 0x00000000000007E8],
    [0x0000000000000000, 0x5439ECF6BF70C600, 0x00000000000003F4],
    [0x00205E71C12F4F15, 0x0C05B368494B8400, 0x00000000000006C0],
    [0xFFDFA18E3ED0B0EA, 0x9C6E2685359607FF, 0x0000000000000128],
    [0x0040BCE3825E9E2B, 0x6F978CE313B57C00, 0x0000000000000597],
    [0x00611B55438DED41, 0xD329665DDE1F7400, 0x000000000000046E],
    [0x008179C704BD3C57, 0x36BB3FD8A8896C00, 0x0000000000000346],
    [0xEB062ACE18383AFC, 0x62F4320962D37D74, 0x0000000000000039],
    1_u64,
    [0xAFB251EC659DFE3D, 0x5D7F49EE6F1D4D66, 0x000000000000008F],
    [0xC6AE150802914D74, 0xC368D55E3A868DE9, 0x0000000000000742],
    5_usize,
    [12, 12, 24, 29, 23, 22, 7, 27, 19, 3, 10, 17, 30, 3],
    14_usize,
    [6, 6, 28, 30, 11, 27, 19, 29, 25, 1, 21, 8, 31, 1],
    14_usize,
    4246015611_u64,
    212669779836060700_u64,
);

// ========================================================
// Definitions of extension fields above the base fields
// GF(p^2) with modulus x^2 + 1 = 0 (using p = 3 mod 4)
// ========================================================

crate::finitefield::fp2_gen::define_fp2_core!(
    Fp139Ext,
    super::Fp139,
    [0x210605607B24DEC5, 0x3D882398A1EFE806, 0x0000000000000201]
);

#[cfg(test)]
mod tests {
    use crate::fields::Fp139;
    use crate::fields::Fp139Ext;

    crate::finitefield::fp_gen::define_fp_tests!(Fp139);
    crate::finitefield::fp2_gen::define_fp2_tests!(Fp139, Fp139Ext);
}
