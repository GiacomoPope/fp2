mod dlp_tests {
    use ::sha2::Digest as _;

    const MODULUS: [u64; 4] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x04FFFFFFFFFFFFFF,
    ];
    fp2::define_fp2_from_modulus!(typename = FpExt, base_typename = Fp, modulus = MODULUS,);

    const X0_BYTES: [u8; 32] = [
        95, 51, 176, 225, 246, 188, 81, 242, 105, 159, 184, 190, 205, 23, 116, 243, 24, 244, 58, 0,
        90, 226, 76, 114, 59, 185, 186, 171, 58, 98, 172, 2,
    ];
    const X1_BYTES: [u8; 32] = [
        181, 108, 226, 218, 242, 204, 118, 176, 75, 251, 210, 112, 60, 175, 196, 112, 193, 69, 49,
        173, 104, 121, 61, 24, 167, 100, 181, 122, 60, 23, 23, 0,
    ];
    const X: FpExt = FpExt::const_decode_no_check(&X0_BYTES, &X1_BYTES);

    #[test]
    fn test_element_order() {
        // ensure x is an element of order 2^248
        let mut x = X;

        // Compute x^(2^127) this should be equal to -1
        for _ in 0..247 {
            x.set_square();
        }
        assert!(x.equals(&FpExt::MINUS_ONE) == u32::MAX);

        // Assert x has order exactly 2^128
        x.set_square();
        assert!(x.equals(&FpExt::ONE) == u32::MAX);
    }

    // Compute x^a for a 128-bit random a
    fn compute_challenge(x: &FpExt, y: &mut FpExt) {
        // We hash the encoding of y to create a new y = x^exp
        let mut exp = [0u8; 31];
        let mut sh = ::sha2::Sha256::new();
        sh.update(y.encode());
        exp.copy_from_slice(&sh.finalize_reset()[0..31]);

        *y = x.pow(&exp, 248);
    }

    #[test]
    fn test_dlp_n() {
        // x is a precomputed element of order 2^248
        let x = X;
        let mut y = FpExt::ONE;

        // Run the test 100 times
        for _ in 0..25 {
            compute_challenge(&x, &mut y);

            let (exp, check) = x.solve_dlp_2e(&y, 248, None);
            let z = x.pow(&exp, 248);
            assert!(check == u32::MAX);
            assert!(z.equals(&y) == u32::MAX);
        }
    }

    #[test]
    fn test_dlp_n_with_table() {
        // x is a precomputed element of order 2^248
        let x = X;
        let mut y = FpExt::ONE;

        let (dlp_table, ele_table, check) = x.precompute_dlp_tables(248);
        assert!(check == u32::MAX);

        for _ in 0..25 {
            compute_challenge(&x, &mut y);

            let (exp, check) = x.solve_dlp_2e(&y, 248, Some((&dlp_table, &ele_table)));
            let z = x.pow(&exp, 248);
            assert!(check == u32::MAX);
            assert!(z.equals(&y) == u32::MAX);
        }
    }
}
