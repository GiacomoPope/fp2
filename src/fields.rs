#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// ========================================================
// Definitions of base fields GF(p) = Z / pZ
// Constants defined are for the macro and generated from
// gen_fp.sage
// ========================================================

// p = 2^74 * 3^41 - 1
pub mod Fp139 {
    const N: usize = 3;
    const BITLEN: usize = 139;
    const MODULUS: [u64; N] = [0xFFFFFFFFFFFFFFFF, 0xA873D9ED7EE18BFF, 0x00000000000007E8];
    const HALF_MODULUS: [u64; N] = [0x0000000000000000, 0x5439ECF6BF70C600, 0x00000000000003F4];
    const R_VAL: [u64; N] = [0x00205E71C12F4F15, 0x0C05B368494B8400, 0x00000000000006C0];
    const MINUS_R_VAL: [u64; N] = [0xFFDFA18E3ED0B0EA, 0x9C6E2685359607FF, 0x0000000000000128];
    const DR_VAL: [u64; N] = [0x0040BCE3825E9E2B, 0x6F978CE313B57C00, 0x0000000000000597];
    const TR_VAL: [u64; N] = [0x00611B55438DED41, 0xD329665DDE1F7400, 0x000000000000046E];
    const QR_VAL: [u64; N] = [0x008179C704BD3C57, 0x36BB3FD8A8896C00, 0x0000000000000346];
    const R2_VAL: [u64; N] = [0xEB062ACE18383AFC, 0x62F4320962D37D74, 0x0000000000000039];
    const P0I: u64 = 1;
    const TFIXDIV_VAL: [u64; N] = [0xAFB251EC659DFE3D, 0x5D7F49EE6F1D4D66, 0x000000000000008F];
    const TDEC_VAL: [u64; N] = [0xC6AE150802914D74, 0xC368D55E3A868DE9, 0x0000000000000742];
    const WIN_LEN: usize = 5;
    const SQRT_EH: [u8; 14] = [12, 12, 24, 29, 23, 22, 7, 27, 19, 3, 10, 17, 30, 3];
    const SQRT_EL: usize = 14;
    const FOURTH_ROOT_EH: [u8; 14] = [6, 6, 28, 30, 11, 27, 19, 29, 25, 1, 21, 8, 31, 1];
    const FOURTH_ROOT_EL: usize = 14;
    const P1: u64 = 4246015611;
    const P1DIV_M: u64 = 212669779836060700;

    crate::finitefield::fp_gen::define_fp_core! {}

    #[cfg(test)]
    mod tests {
        crate::finitefield::fp_gen::define_fp_tests! {}
    }
}

// p = 5*2^458 - 1
pub mod Fp248 {
    const N: usize = 4;
    const BITLEN: usize = 251;
    const MODULUS: [u64; N] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x04FFFFFFFFFFFFFF,
    ];
    const HALF_MODULUS: [u64; N] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0280000000000000,
    ];
    const R_VAL: [u64; N] = [
        0x0000000000000033,
        0x0000000000000000,
        0x0000000000000000,
        0x0100000000000000,
    ];
    const MINUS_R_VAL: [u64; N] = [
        0xFFFFFFFFFFFFFFCC,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x03FFFFFFFFFFFFFF,
    ];
    const DR_VAL: [u64; N] = [
        0x0000000000000066,
        0x0000000000000000,
        0x0000000000000000,
        0x0200000000000000,
    ];
    const TR_VAL: [u64; N] = [
        0x0000000000000099,
        0x0000000000000000,
        0x0000000000000000,
        0x0300000000000000,
    ];
    const QR_VAL: [u64; N] = [
        0x00000000000000CC,
        0x0000000000000000,
        0x0000000000000000,
        0x0400000000000000,
    ];
    const R2_VAL: [u64; N] = [
        0x3333333333333D70,
        0x3333333333333333,
        0x3333333333333333,
        0x0333333333333333,
    ];
    const P0I: u64 = 1;
    const TFIXDIV_VAL: [u64; N] = [
        0x49BA5E3BCD35A858,
        0xF7CED916872B020C,
        0x72B020C49BA5E353,
        0x025E353F7CED9168,
    ];
    const TDEC_VAL: [u64; N] = [
        0x3333333333333333,
        0x3333333333333333,
        0x3333333333333333,
        0x0100000000000033,
    ];
    const WIN_LEN: usize = 5;
    const SQRT_EH: [u8; 1] = [10];
    const SQRT_EL: usize = 49;
    const FOURTH_ROOT_EH: [u8; 1] = [5];
    const FOURTH_ROOT_EL: usize = 49;
    const P1: u64 = 2684354559;
    const P1DIV_M: u64 = 11068046455220847252;

    crate::finitefield::fp_gen::define_fp_core! {}

    #[cfg(test)]
    mod tests {
        crate::finitefield::fp_gen::define_fp_tests! {}
    }
}

// p = 2^128 * 3^76 * 5 * 7 - 1
pub mod Fp254 {
    const N: usize = 4;
    const BITLEN: usize = 254;
    const MODULUS: [u64; N] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xB52F88A2BBB638F2,
        0x300C882522D1C193,
    ];
    const HALF_MODULUS: [u64; N] = [
        0x0000000000000000,
        0x8000000000000000,
        0xDA97C4515DDB1C79,
        0x180644129168E0C9,
    ];
    const R_VAL: [u64; N] = [
        0x0000000000000005,
        0x0000000000000000,
        0x761254D25570E341,
        0x0FC1574651E7381D,
    ];
    const MINUS_R_VAL: [u64; N] = [
        0xFFFFFFFFFFFFFFFA,
        0xFFFFFFFFFFFFFFFF,
        0x3F1D33D0664555B1,
        0x204B30DED0EA8976,
    ];
    const DR_VAL: [u64; N] = [
        0x000000000000000A,
        0x0000000000000000,
        0xEC24A9A4AAE1C682,
        0x1F82AE8CA3CE703A,
    ];
    const TR_VAL: [u64; N] = [
        0x000000000000000F,
        0x0000000000000000,
        0x6236FE770052A9C3,
        0x2F4405D2F5B5A858,
    ];
    const QR_VAL: [u64; N] = [
        0x0000000000000015,
        0x0000000000000000,
        0x2319CAA69A0D5411,
        0x0EF8D4F424CB1EE2,
    ];
    const R2_VAL: [u64; N] = [
        0xE68D0176D1DE3F65,
        0xD5C29C23725CE9C4,
        0x333FEFEC3ADA8206,
        0x1CB17200F07FFDEA,
    ];
    const P0I: u64 = 1;
    const TFIXDIV_VAL: [u64; N] = [
        0x9DB4AE4A664B1867,
        0xF6DA2E7D8CD19F7B,
        0x5538A8E3E2FB8DD6,
        0x2B17BAA50A8EB9A9,
    ];
    const TDEC_VAL: [u64; N] = [
        0xD5C29C23725CE9C4,
        0xA60DA3D7E77CC6E5,
        0xBA5A0AE3EAA05CD2,
        0x2B45B97EB36F799D,
    ];
    const WIN_LEN: usize = 5;
    const SQRT_EH: [u8; 26] = [
        6, 15, 28, 24, 22, 27, 21, 8, 17, 24, 23, 20, 22, 19, 12, 16, 3, 13, 17, 20, 4, 8, 4, 3, 0,
        3,
    ];
    const SQRT_EL: usize = 25;
    const FOURTH_ROOT_EH: [u8; 26] = [
        19, 7, 14, 12, 27, 29, 10, 20, 8, 28, 11, 10, 27, 9, 6, 24, 17, 22, 8, 10, 2, 4, 18, 1, 16,
        1,
    ];
    const FOURTH_ROOT_EL: usize = 25;
    const P1: u64 = 3224510612;
    const P1DIV_M: u64 = 6123856568576173817;

    crate::finitefield::fp_gen::define_fp_core! {}

    #[cfg(test)]
    mod tests {
        crate::finitefield::fp_gen::define_fp_tests! {}
    }
}

// p = 2^604 * 3^363 - 1
pub mod Fp1180 {
    const N: usize = 19;
    const BITLEN: usize = 1180;
    const MODULUS: [u64; N] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xE131E867AFFFFFFF,
        0x7C48CF51A9564BEC,
        0x8BF48C8A7CF7213B,
        0x65CBAC74648ABF61,
        0x8AE12C2210FA3302,
        0xB4B39723007054AA,
        0x5ACD85FCDA9F947A,
        0x9B03D875F853CB9F,
        0x152E5149E72DDE4E,
        0x000000000A22C3A7,
    ];
    const HALF_MODULUS: [u64; N] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x7098F433D8000000,
        0xBE2467A8D4AB25F6,
        0xC5FA46453E7B909D,
        0x32E5D63A32455FB0,
        0x45709611087D1981,
        0x5A59CB9180382A55,
        0xAD66C2FE6D4FCA3D,
        0x4D81EC3AFC29E5CF,
        0x8A9728A4F396EF27,
        0x00000000051161D3,
    ];
    const R_VAL: [u64; N] = [
        0x0000001941CBBBE2,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0xEF4486E6A0000000,
        0x6D1AB85F2076FFAB,
        0xFFC4AE3E90B7EC59,
        0x3BD916A9B1F1FE85,
        0x9C382EF04038F0DD,
        0x8337DE8D8E8011E8,
        0xE916A1DAE2F0A379,
        0x7DB4A7BFC4CB904D,
        0xAAF39465070D49D7,
        0x0000000004FFBB70,
    ];
    const MINUS_R_VAL: [u64; N] = [
        0xFFFFFFE6BE34441D,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xF1ED61810FFFFFFF,
        0x0F2E16F288DF4C40,
        0x8C2FDE4BEC3F34E2,
        0x29F295CAB298C0DB,
        0xEEA8FD31D0C14225,
        0x317BB89571F042C1,
        0x71B6E421F7AEF101,
        0x1D4F30B633883B51,
        0x6A3ABCE4E0209477,
        0x0000000005230836,
    ];
    const DR_VAL: [u64; N] = [
        0x00000032839777C4,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0xDE890DCD40000000,
        0xDA3570BE40EDFF57,
        0xFF895C7D216FD8B2,
        0x77B22D5363E3FD0B,
        0x38705DE08071E1BA,
        0x066FBD1B1D0023D1,
        0xD22D43B5C5E146F3,
        0xFB694F7F8997209B,
        0x55E728CA0E1A93AE,
        0x0000000009FF76E1,
    ];
    const TR_VAL: [u64; N] = [
        0x0000004BC56333A7,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0xEC9BAC4C30000000,
        0xCB0759CBB80EB316,
        0x73597E313530A3D0,
        0x4DBF9788B14B3C30,
        0x49C760AEAFB09F95,
        0xD4F40485AB0FE10F,
        0x60765F93CE3255F1,
        0xDE1A1EC9560EE54A,
        0xEBAC6BE52DF9FF37,
        0x0000000004DC6EAA,
    ];
    const QR_VAL: [u64; N] = [
        0x00000065072EEF89,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0xDBE03332D0000000,
        0x3822122AD885B2C2,
        0x731E2C6FC5E8902A,
        0x8998AE32633D3AB6,
        0xE5FF8F9EEFE99072,
        0x582BE313398FF2F7,
        0x498D016EB122F96B,
        0x5BCEC6891ADA7598,
        0x96A0004A3507490F,
        0x0000000009DC2A1B,
    ];
    const R2_VAL: [u64; N] = [
        0x3656770393DA5F93,
        0x8E37D447697D6600,
        0x1363F7058CA07E73,
        0x8F29BB9715D4AD3A,
        0xF4E98E8669C6FCFE,
        0x9CE5F2FEE9518350,
        0xBEA8ED71ED7D0B1B,
        0xB6F82299422CBF10,
        0x4DC84AB8926C8E8C,
        0x9505ECF91E38A01D,
        0x32B11ADD2F7C4ECD,
        0x604B2825EE6DC153,
        0x8D044C7A62356B8A,
        0x9FBFB4CC9928D3CA,
        0x7F6D22C092F96778,
        0x681DEE94318DE3C6,
        0x340F18451EF10A45,
        0x5A9BD2EDF90CF397,
        0x00000000016C339C,
    ];
    const P0I: u64 = 1;
    const TFIXDIV_VAL: [u64; N] = [
        0x4A1EEECC5F13F5AC,
        0x1DC6E0B9596F2901,
        0x382238902C24C7F5,
        0xD5ECB5694B451EE3,
        0x03C99E98944496F7,
        0x640F48068E54BEEC,
        0x294582FB094F171C,
        0xDD6AF57541ECB76C,
        0x5625F8E49A296654,
        0xC4C51BCDD0172BCE,
        0xB2F40D7CCA6B09B1,
        0x500C0423D789FE88,
        0xF32D6F0B4C44C925,
        0xF9F5AEC2FE9E0F67,
        0x87F04DA775F9F303,
        0x06649BB8B64E8AD0,
        0x05E329E8039F3258,
        0xED3BFD14DC95FCD8,
        0x0000000000D6A84E,
    ];
    const TDEC_VAL: [u64; N] = [
        0x8E37D447697D6600,
        0x1363F7058CA07E73,
        0x8F29BB9715D4AD3A,
        0xF4E98E8669C6FCFE,
        0x9CE5F2FEE9518350,
        0xBEA8ED71ED7D0B1B,
        0xB6F82299422CBF10,
        0x4DC84AB8926C8E8C,
        0xEC48BAD32E38A01D,
        0x979863ECE7D1485E,
        0xBF1AB93C07110954,
        0xD1B1FEAB5569402E,
        0x381AB0285D1CD07D,
        0x57F434CBD90CB421,
        0x9CF0DFB39AC09238,
        0x97F811D3A936A6F1,
        0xA80828CC648375BD,
        0x45CE743EF45AD429,
        0x000000000226C1A9,
    ];
    const WIN_LEN: usize = 5;
    const SQRT_EH: [u8; 116] = [
        12, 15, 6, 20, 7, 6, 1, 7, 27, 23, 4, 11, 5, 21, 17, 26, 19, 17, 4, 30, 13, 7, 1, 25, 29,
        25, 7, 5, 18, 17, 20, 31, 2, 3, 22, 31, 10, 17, 4, 3, 29, 24, 26, 5, 23, 12, 2, 24, 12, 20,
        15, 8, 8, 4, 12, 9, 24, 21, 8, 21, 18, 10, 16, 3, 0, 6, 18, 11, 14, 22, 20, 21, 30, 8, 25,
        15, 10, 27, 28, 15, 1, 27, 12, 13, 29, 19, 11, 30, 20, 16, 31, 26, 1, 27, 3, 24, 6, 29, 4,
        15, 23, 5, 7, 15, 18, 2, 5, 23, 20, 2, 7, 29, 16, 5, 2, 5,
    ];
    const SQRT_EL: usize = 120;
    const FOURTH_ROOT_EH: [u8; 116] = [
        22, 7, 3, 26, 3, 19, 16, 19, 29, 11, 18, 21, 18, 26, 8, 29, 25, 8, 2, 31, 22, 19, 16, 28,
        30, 28, 19, 2, 25, 8, 26, 15, 17, 1, 27, 15, 21, 8, 18, 17, 14, 12, 29, 18, 11, 6, 1, 12,
        6, 26, 7, 4, 4, 2, 22, 4, 28, 10, 20, 10, 9, 5, 24, 1, 0, 3, 25, 5, 7, 11, 26, 10, 15, 20,
        28, 7, 21, 13, 30, 23, 16, 13, 22, 22, 30, 25, 5, 15, 10, 24, 15, 29, 16, 29, 1, 12, 19,
        14, 18, 23, 27, 18, 19, 7, 9, 17, 18, 11, 10, 17, 19, 14, 24, 2, 17, 2,
    ];
    const FOURTH_ROOT_EL: usize = 120;
    const P1: u64 = 2720807537;
    const P1DIV_M: u64 = 10672611645814220636;

    crate::finitefield::fp_gen::define_fp_core! {}

    #[cfg(test)]
    mod tests {
        crate::finitefield::fp_gen::define_fp_tests! {}
    }
}

// ========================================================
// Definitions of extension fields above the base fields
// GF(p^2) with modulus x^2 + 1 = 0 (using p = 3 mod 4)
// ========================================================

pub mod Fp139Ext {
    use super::Fp139::Fp;
    const NQR_RE: Fp = Fp::new([0x210605607B24DEC5, 0x3D882398A1EFE806, 0x0000000000000201]);

    crate::finitefield::fp2_gen::define_fp2_core! {}
    #[cfg(test)]
    mod tests {
        crate::finitefield::fp2_gen::define_fp2_tests! {}
    }
}

pub mod Fp248Ext {
    use super::Fp248::Fp;
    const NQR_RE: Fp = Fp::new([
        0xAF32723ECE082DD0,
        0xACEA4D9296C5A390,
        0x59B3715FE4B31D02,
        0x03A726359E3001D0,
    ]);

    crate::finitefield::fp2_gen::define_fp2_core! {}
    #[cfg(test)]
    mod tests {
        crate::finitefield::fp2_gen::define_fp2_tests! {}
    }
}

pub mod Fp254Ext {
    use super::Fp254::Fp;
    const NQR_RE: Fp = Fp::new([
        0x19067C2EE042CAE9,
        0x7FAA1A2A81781083,
        0xF0EE8CA2EBCDDCE4,
        0x2561C37B60330EF1,
    ]);

    crate::finitefield::fp2_gen::define_fp2_core! {}
    #[cfg(test)]
    mod tests {
        crate::finitefield::fp2_gen::define_fp2_tests! {}
    }
}

pub mod Fp1180Ext {
    use super::Fp1180::Fp;
    const NQR_RE: Fp = Fp::new([
        0x63137E890AF23EDA,
        0x6A672094A0E160C1,
        0xEB5A827F45228589,
        0x2FE79A68FBBEF719,
        0x28C3DB85CFE5789C,
        0x2E5D89C759D0FC9C,
        0xA54070FEA344A8DB,
        0x3514D875E06F48F0,
        0x7927B26536CA8041,
        0x36A2A9D2D5D23456,
        0x0F35669BD2FD686D,
        0xA78F2E1DE4A67DE5,
        0x9A3776D04636D47A,
        0x0B4DC0C010EFC8B0,
        0xD3EAC83F9DBEE0C7,
        0x0DC287C96E066E4C,
        0x6B1A54D9E3FD0803,
        0xD71340E2C821572C,
        0x00000000082DDB95,
    ]);

    crate::finitefield::fp2_gen::define_fp2_core! {}

    // skip these tests for now, slow.
    #[cfg(test)]
    mod tests {
        // crate::finitefield::fp2_gen::define_fp2_tests! {} // slow
    }
}
