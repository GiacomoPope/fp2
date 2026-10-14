#[cfg(feature = "simd")]
use fp2::__private::MontgomeryField;
use fp2::packed::{PackedField, PackedOperation};
use fp2::traits::Fq;

macro_rules! field {
    ($name:ident, $len:literal, $modulus:expr) => {
        mod $name {
            pub const MODULUS: [u64; $len] = $modulus;
            fp2::define_fp_core!(typename = Field, modulus = MODULUS,);
        }
    };
}

field!(
    supported,
    4,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x04ff_ffff_ffff_ffff,
    ]
);
field!(unsupported_tiny, 1, [6143]);
field!(
    unsupported_headroom,
    2,
    [0xffff_ffff_ffff_ffff, 0x7fff_ffff_ffff_ffff]
);

struct Smoke;

impl<F: Fq> PackedOperation<F> for Smoke {
    type Output = bool;

    fn run<C: PackedField<Scalar = F>>(self, context: C) -> bool {
        let a = [F::ZERO, F::ONE, F::MINUS_ONE, F::from(0x1234_5678u64)];
        let b = [F::TWO, -F::TWO, F::ONE, F::MINUS_ONE];
        let pa = context.pack(a);
        let pb = context.pack(b);
        let result = context.unpack(&context.butterfly::<1>(&context.mul(&pa, &pb)));
        let product: [F; 4] = core::array::from_fn(|i| a[i] * b[i]);
        let expected = [
            product[0] + product[1],
            product[0] - product[1],
            product[2] + product[3],
            product[2] - product[3],
        ];
        for (actual, expected) in result.into_iter().zip(expected) {
            assert_eq!(actual.equals(&expected), u32::MAX);
        }
        context.is_accelerated()
    }
}

#[test]
fn generated_field_dispatches_packed_operation() {
    let accelerated = supported::Field::with_packed(Smoke);
    #[cfg(feature = "simd")]
    assert_eq!(accelerated, !fearless_simd::Level::new().is_fallback());
    #[cfg(not(feature = "simd"))]
    assert!(!accelerated);
}

#[test]
fn unsupported_moduli_use_serial_fallback() {
    assert!(!unsupported_tiny::Field::with_packed(Smoke));
    assert!(!unsupported_headroom::Field::with_packed(Smoke));
}

#[cfg(feature = "simd")]
#[test]
fn bridge_preserves_exact_montgomery_digits() {
    type F = supported::Field;
    struct RoundTrip(F);
    impl PackedOperation<F> for RoundTrip {
        type Output = F;

        fn run<C: PackedField<Scalar = F>>(self, context: C) -> F {
            context.unpack(&context.pack([self.0; 4]))[0]
        }
    }

    for value in [F::ZERO, F::ONE, F::MINUS_ONE, F::from(0x1234_5678u64)] {
        let mut expected = [0u32; 2 * F::N];
        value.write_montgomery_digits(&mut expected);
        let mut actual = [0u32; 2 * F::N];
        F::with_packed(RoundTrip(value)).write_montgomery_digits(&mut actual);
        assert_eq!(actual, expected);
    }
}
