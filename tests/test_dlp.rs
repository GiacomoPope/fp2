mod dlp_tests {
    use ::sha2::Digest as _;

    pub static MODULUS: [u64; 4] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x04FFFFFFFFFFFFFF,
    ];
    pub static X_STR: &str = "5f33b0e1f6bc51f2699fb8becd1774f318f43a005ae24c723bb9baab3a62ac02b56ce2daf2cc76b04bfbd2703cafc470c14531ad68793d18a764b57a3c171700";

    fp2::define_fp2_from_modulus!(typename = FpExt, base_typename = Fp, modulus = MODULUS,);

    #[test]
    fn test_element_order() {
        // ensure x is an element of order 2^248
        let (mut x, r) = FpExt::decode(&hex::decode(X_STR).unwrap());
        assert!(r == u32::MAX);

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
        let (x, r) = FpExt::decode(&hex::decode(X_STR).unwrap());
        assert!(r == u32::MAX);

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
        let (x, r) = FpExt::decode(&hex::decode(X_STR).unwrap());
        assert!(r == u32::MAX);

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
