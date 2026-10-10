use core::any::type_name;
use core::marker::PhantomData;

use fearless_simd::{Simd, prelude::*, u64x4};

use super::*;
use crate::packed::{PackedField, Serial};

macro_rules! field {
    ($name:ident, $len:literal, $modulus:expr) => {
        #[allow(unused_assignments, unused_variables)]
        pub(super) mod $name {
            pub const MODULUS: [u64; $len] = $modulus;
            crate::define_fp_core!(typename = Field, modulus = MODULUS,);
        }
    };
}

field!(
    radix32_field,
    2,
    [0xffff_ffff_ffff_ffff, 0x04ff_ffff_ffff_ffff]
);
field!(
    insufficient_headroom,
    2,
    [0xffff_ffff_ffff_ffff, 0x4000_0000_0000_0000]
);
field!(
    mike_fast_i,
    5,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x278f_ffff_ffff_ffff,
    ]
);
field!(
    mike_fast_iii,
    8,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x0000_0009_43ff_ffff,
    ]
);
field!(
    mike_fast_v,
    10,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x13cf_ffff_ffff_ffff,
    ]
);
field!(
    mike_conservative_i,
    6,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x1d3f_ffff_ffff_ffff,
    ]
);
field!(
    mike_conservative_iii,
    9,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x133f_ffff_ffff_ffff,
    ]
);
field!(
    mike_conservative_v,
    12,
    [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x0a3f_ffff_ffff_ffff,
    ]
);
fn next_u64(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

fn random_field<F: Fq>(state: &mut u64) -> F {
    let mut bytes = vec![0u8; F::ENCODED_LENGTH + 8];
    for chunk in bytes.chunks_mut(8) {
        let word = next_u64(state).to_le_bytes();
        let len = chunk.len();
        chunk.copy_from_slice(&word[..len]);
    }
    F::decode_reduce(&bytes)
}

fn assert_lanes_eq<F: Fq>(actual: [F; 4], expected: [F; 4]) {
    for (lane, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert_eq!(
            actual.equals(&expected),
            u32::MAX,
            "scalar mismatch in lane {lane}"
        );
    }
}

fn check_result<F, C, V>(context: &C, actual: &C::Packed, expected: [F; 4], assert_valid: &V)
where
    F: Fq,
    C: PackedField<Scalar = F>,
    V: Fn(&C::Packed),
{
    assert_valid(actual);
    assert_lanes_eq(context.unpack(actual), expected);
}

fn endpoint_rotations<C: PackedField>(context: &C, endpoints: C::Packed) -> [C::Packed; 4] {
    [
        endpoints,
        context.gather::<1, 2, 3, 0>(&[endpoints]),
        context.gather::<2, 3, 0, 1>(&[endpoints]),
        context.gather::<3, 0, 1, 2>(&[endpoints]),
    ]
}

pub(super) fn exercise_arithmetic<F, C, V>(context: C, endpoints: C::Packed, assert_valid: V)
where
    F: Fq,
    C: PackedField<Scalar = F>,
    V: Fn(&C::Packed),
{
    context.vectorize(
        #[inline(always)]
        || {
            let endpoint_packs = endpoint_rotations(&context, endpoints);
            for a in endpoint_packs {
                let aa = context.unpack(&a);
                check_result(
                    &context,
                    &context.square(&a),
                    aa.map(Fq::square),
                    &assert_valid,
                );
                check_result(
                    &context,
                    &context.butterfly::<1>(&a),
                    [aa[0] + aa[1], aa[0] - aa[1], aa[2] + aa[3], aa[2] - aa[3]],
                    &assert_valid,
                );
                check_result(
                    &context,
                    &context.butterfly::<2>(&a),
                    [aa[0] + aa[2], aa[1] + aa[3], aa[0] - aa[2], aa[1] - aa[3]],
                    &assert_valid,
                );
                for b in endpoint_packs {
                    let bb = context.unpack(&b);
                    check_result(
                        &context,
                        &context.add(&a, &b),
                        core::array::from_fn(|i| aa[i] + bb[i]),
                        &assert_valid,
                    );
                    check_result(
                        &context,
                        &context.sub(&a, &b),
                        core::array::from_fn(|i| aa[i] - bb[i]),
                        &assert_valid,
                    );
                    check_result(
                        &context,
                        &context.mul(&a, &b),
                        core::array::from_fn(|i| aa[i] * bb[i]),
                        &assert_valid,
                    );
                }
            }

            let mut state = 0x6670_325f_7369_6d64;
            for _ in 0..8 {
                let a = core::array::from_fn(|_| random_field::<F>(&mut state));
                let b = core::array::from_fn(|_| random_field::<F>(&mut state));
                let c = core::array::from_fn(|_| random_field::<F>(&mut state));
                let pa = context.pack(a);
                let pb = context.pack(b);
                let pc = context.pack(c);

                check_result(
                    &context,
                    &context.add(&pa, &pb),
                    core::array::from_fn(|i| a[i] + b[i]),
                    &assert_valid,
                );
                check_result(
                    &context,
                    &context.sub(&pa, &pb),
                    core::array::from_fn(|i| a[i] - b[i]),
                    &assert_valid,
                );
                check_result(
                    &context,
                    &context.mul(&pa, &pb),
                    core::array::from_fn(|i| a[i] * b[i]),
                    &assert_valid,
                );
                check_result(
                    &context,
                    &context.square(&pa),
                    a.map(Fq::square),
                    &assert_valid,
                );
                check_result(
                    &context,
                    &context.gather::<7, 0, 5, 2>(&[pa, pb]),
                    [b[3], a[0], b[1], a[2]],
                    &assert_valid,
                );

                let mut packed = pa;
                let mut expected = a;
                for step in 0..48 {
                    packed = match step % 6 {
                        0 => {
                            expected = core::array::from_fn(|i| expected[i] + b[i]);
                            context.add(&packed, &pb)
                        }
                        1 => {
                            expected = core::array::from_fn(|i| expected[i] - c[i]);
                            context.sub(&packed, &pc)
                        }
                        2 => {
                            expected = core::array::from_fn(|i| expected[i] * b[i]);
                            context.mul(&packed, &pb)
                        }
                        3 => {
                            expected = expected.map(Fq::square);
                            context.square(&packed)
                        }
                        4 => {
                            expected = [
                                expected[0] + expected[1],
                                expected[0] - expected[1],
                                expected[2] + expected[3],
                                expected[2] - expected[3],
                            ];
                            context.butterfly::<1>(&packed)
                        }
                        _ => {
                            expected = [
                                expected[0] + expected[2],
                                expected[1] + expected[3],
                                expected[0] - expected[2],
                                expected[1] - expected[3],
                            ];
                            context.butterfly::<2>(&packed)
                        }
                    };
                    check_result(&context, &packed, expected, &assert_valid);
                }
            }
        },
    );
}

fn radix32_endpoints<F: MontgomeryField, S: Simd, const L: usize>(
    simd: S,
) -> radix32::Packed<F, S, L> {
    assert_eq!(L, 2 * F::MODULUS.len());
    let modulus: [u32; L] = core::array::from_fn(|i| {
        let word = F::MODULUS[i / 2];
        if i % 2 == 0 {
            word as u32
        } else {
            (word >> 32) as u32
        }
    });
    let mut p_minus_one = modulus;
    p_minus_one[0] -= 1;
    let mut carry = 0u64;
    let mut twice_p_minus_one = modulus.map(|digit| {
        let sum = 2 * u64::from(digit) + carry;
        carry = sum >> 32;
        sum as u32
    });
    assert_eq!(carry, 0);
    twice_p_minus_one[0] -= 1;
    radix32::Packed {
        limbs: core::array::from_fn(|i| {
            u64x4::simd_from(
                simd,
                [
                    0,
                    u64::from(modulus[i]),
                    u64::from(p_minus_one[i]),
                    u64::from(twice_p_minus_one[i]),
                ],
            )
        }),
        field: PhantomData,
    }
}

fn assert_radix32_valid<F, S, const L: usize, const LOW: usize>(
    context: &radix32::Context<F, S, L, LOW>,
    value: &radix32::Packed<F, S, L>,
) where
    F: MontgomeryField,
    S: Simd,
{
    let digits: [[u64; 4]; L] = value.limbs.map(Into::into);
    let twice: [[u64; 4]; L] = context.twice_modulus.limbs.map(Into::into);
    for lane in 0..4 {
        let mut borrow = false;
        for (digits, twice) in digits.iter().zip(twice.iter()) {
            assert!(digits[lane] <= u64::from(u32::MAX));
            borrow = digits[lane]
                .overflowing_sub(twice[lane] + u64::from(borrow))
                .1;
        }
        assert!(borrow, "radix-32 lane {lane} is not below 2p");
    }
}

fn exercise_radix32<F, S, const L: usize, const LOW: usize>(simd: S)
where
    F: MontgomeryField,
    S: Simd,
{
    let context =
        radix32::Context::<F, S, L, LOW>::new(simd).expect("radix-32 fixture must be supported");
    let endpoints = radix32_endpoints(simd);
    let endpoint_values = context.unpack(&endpoints);
    assert_eq!(endpoint_values[0].is_zero(), u32::MAX);
    assert_eq!(endpoint_values[1].is_zero(), u32::MAX);
    assert_eq!(endpoint_values[2].equals(&endpoint_values[3]), u32::MAX);
    exercise_arithmetic(context, endpoints, |value| {
        assert_radix32_valid(&context, value)
    });
}

fn exercise_all_radix32_fields<S: Simd>(simd: S) {
    exercise_radix32::<radix32_field::Field, _, 4, 0>(simd);
    exercise_radix32::<radix32_field::Field, _, 4, 3>(simd);
    exercise_radix32::<mike_fast_i::Field, _, 10, 0>(simd);
    exercise_radix32::<mike_fast_i::Field, _, 10, 9>(simd);
    exercise_radix32::<mike_fast_iii::Field, _, 16, 0>(simd);
    exercise_radix32::<mike_fast_iii::Field, _, 16, 14>(simd);
    exercise_radix32::<mike_fast_v::Field, _, 20, 0>(simd);
    exercise_radix32::<mike_fast_v::Field, _, 20, 19>(simd);
    exercise_radix32::<mike_conservative_i::Field, _, 12, 0>(simd);
    exercise_radix32::<mike_conservative_i::Field, _, 12, 11>(simd);
    exercise_radix32::<mike_conservative_iii::Field, _, 18, 0>(simd);
    exercise_radix32::<mike_conservative_iii::Field, _, 18, 17>(simd);
    exercise_radix32::<mike_conservative_v::Field, _, 24, 0>(simd);
    exercise_radix32::<mike_conservative_v::Field, _, 24, 23>(simd);
}

#[test]
fn serial_arithmetic() {
    type F = radix32_field::Field;
    let context = Serial::<F>::new();
    let endpoints = context.pack([F::ZERO, F::ZERO, F::MINUS_ONE, F::MINUS_ONE]);
    exercise_arithmetic(context, endpoints, |_| {});
}

#[test]
fn radix32_arithmetic_at_each_available_simd_level() {
    #[cfg(feature = "test-simd-fallback")]
    exercise_all_radix32_fields(fearless_simd::Fallback::new());

    let level = fearless_simd::Level::new();
    #[cfg(target_arch = "aarch64")]
    if let Some(simd) = level.as_neon() {
        exercise_all_radix32_fields(simd);
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if let Some(simd) = level.as_sse2() {
            exercise_all_radix32_fields(simd);
        }
        if let Some(simd) = level.as_sse4_2() {
            exercise_all_radix32_fields(simd);
        }
        if let Some(simd) = level.as_avx2() {
            exercise_all_radix32_fields(simd);
        }
        if let Some(simd) = level.as_avx512() {
            exercise_all_radix32_fields(simd);
        }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if let Some(simd) = level.as_wasm_simd128() {
        exercise_all_radix32_fields(simd);
    }
}

#[test]
fn radix32_constructor_checks_each_guard() {
    fearless_simd::dispatch!(fearless_simd::Level::new(), simd => {
        assert!(radix32::Context::<radix32_field::Field, _, 4, 3>::new(simd).is_some());
        assert!(radix32::Context::<radix32_field::Field, _, 2, 0>::new(simd).is_none(), "wrong digit count");
        assert!(radix32::Context::<radix32_field::Field, _, 4, 2>::new(simd).is_none(), "wrong all-ones prefix");
        assert!(radix32::Context::<insufficient_headroom::Field, _, 4, 2>::new(simd).is_none(), "modulus lacks two bits of headroom");
    });
}

#[derive(Debug, Eq, PartialEq)]
struct BackendProbe {
    name: &'static str,
    accelerated: bool,
}

impl<F: Fq> PackedOperation<F> for BackendProbe {
    type Output = Self;

    fn run<C: PackedField<Scalar = F>>(mut self, context: C) -> Self {
        self.name = type_name::<C>();
        self.accelerated = context.is_accelerated();
        self
    }
}

fn probe<F: Fq>() -> BackendProbe {
    BackendProbe {
        name: type_name::<Serial<F>>(),
        accelerated: false,
    }
}

#[test]
fn dispatch_uses_radix32_then_serial_fallback() {
    type F = radix32_field::Field;
    let level = fearless_simd::Level::new();
    let selected = dispatch_at::<F, _, 4, 3>(level, probe::<F>());
    if level.is_fallback() {
        assert!(selected.name.contains("packed::Serial"));
        assert!(!selected.accelerated);
    } else {
        assert!(selected.name.contains("simd::radix32::Context"));
        assert!(selected.accelerated);
    }

    let rejected = dispatch_at::<F, _, 2, 0>(level, probe::<F>());
    assert!(rejected.name.contains("packed::Serial"));
    assert!(!rejected.accelerated);

    #[cfg(feature = "test-simd-fallback")]
    {
        let fallback = dispatch_at::<F, _, 4, 3>(fearless_simd::Level::fallback(), probe::<F>());
        assert!(fallback.name.contains("packed::Serial"));
        assert!(!fallback.accelerated);
    }
}

#[test]
fn ci_expected_simd_backend_is_available() {
    let Ok(expected) = std::env::var("FP2_EXPECT_SIMD_BACKEND") else {
        return;
    };
    let level = fearless_simd::Level::new();
    match expected.as_str() {
        "avx2" => {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            assert!(level.as_avx2().is_some(), "CI host does not provide AVX2");
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            panic!("CI expected AVX2 on a non-x86 host");
        }
        "neon" => {
            #[cfg(target_arch = "aarch64")]
            assert!(level.as_neon().is_some(), "CI host does not provide NEON");
            #[cfg(not(target_arch = "aarch64"))]
            panic!("CI expected NEON on a non-AArch64 host");
        }
        other => panic!("unknown FP2_EXPECT_SIMD_BACKEND value: {other}"),
    }
}
