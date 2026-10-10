use core::marker::PhantomData;

use fearless_simd::{Simd, prelude::*, u32x4};

use super::*;
use crate::packed::{PackedField, PackedOperation};
use crate::simd::tests::{
    exercise_arithmetic, mike_conservative_i, mike_conservative_iii, mike_conservative_v,
    mike_fast_i, mike_fast_iii, mike_fast_v,
};
use crate::traits::Fq;

#[allow(unused_assignments, unused_variables)]
mod incompatible_sparse_shape {
    pub const MODULUS: [u64; 5] = [
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_fffe,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x278f_ffff_ffff_ffff,
    ];
    crate::define_fp_core!(typename = Field, modulus = MODULUS,);
}

fn subtract_one<const K: usize, const W: usize>(mut digits: [u32; K]) -> [u32; K] {
    let mask = (1u32 << W) - 1;
    for digit in &mut digits {
        if *digit == 0 {
            *digit = mask;
        } else {
            *digit -= 1;
            break;
        }
    }
    digits
}

fn endpoints<F, S, const L: usize, const LOW: usize, const K: usize, const W: usize>(
    context: &UnsaturatedContext<F, S, L, LOW, K, W>,
) -> UnsaturatedPacked<F, S, K>
where
    F: MontgomeryField,
    S: Simd,
{
    let p_minus_one = subtract_one::<K, W>(context.modulus_digits);
    let twice_p_minus_one = subtract_one::<K, W>(context.twice_modulus_digits);
    UnsaturatedPacked {
        limbs: core::array::from_fn(|i| {
            u32x4::simd_from(
                context.simd,
                [
                    0,
                    context.modulus_digits[i],
                    p_minus_one[i],
                    twice_p_minus_one[i],
                ],
            )
        }),
        field: PhantomData,
    }
}

fn assert_valid<F, S, const L: usize, const LOW: usize, const K: usize, const W: usize>(
    context: &UnsaturatedContext<F, S, L, LOW, K, W>,
    value: &UnsaturatedPacked<F, S, K>,
) where
    F: MontgomeryField,
    S: Simd,
{
    let mask = (1u32 << W) - 1;
    let digits: [[u32; 4]; K] = value.limbs.map(Into::into);
    for lane in 0..4 {
        let mut borrow = false;
        for (i, (digits, modulus)) in digits
            .iter()
            .zip(context.twice_modulus_digits.iter())
            .enumerate()
        {
            assert!(
                digits[lane] <= mask,
                "unnormalized unsaturated digit {i} in lane {lane}"
            );
            borrow = u64::from(digits[lane])
                .overflowing_sub(u64::from(*modulus) + u64::from(borrow))
                .1;
        }
        assert!(borrow, "unsaturated lane {lane} is not below 2p");
    }
}

fn exercise<F, S, const L: usize, const LOW: usize, const K: usize, const W: usize>(simd: S)
where
    F: MontgomeryField,
    S: Simd,
    UnsaturatedContext<F, S, L, LOW, K, W>:
        PackedField<Scalar = F, Packed = UnsaturatedPacked<F, S, K>>,
{
    let context = UnsaturatedContext::<F, S, L, LOW, K, W>::new(simd)
        .expect("unsaturated fixture must match its layout");
    let endpoints = endpoints(&context);
    let endpoint_values = context.unpack(&endpoints);
    assert_eq!(endpoint_values[0].is_zero(), u32::MAX);
    assert_eq!(endpoint_values[1].is_zero(), u32::MAX);
    assert_eq!(endpoint_values[2].equals(&endpoint_values[3]), u32::MAX);
    exercise_arithmetic(context, endpoints, |value| assert_valid(&context, value));
}

fn exercise_all_layouts<S: Simd>(simd: S)
where
    UnsaturatedContext<mike_fast_i::Field, S, 10, 9, 11, 30>: PackedField<
            Scalar = mike_fast_i::Field,
            Packed = UnsaturatedPacked<mike_fast_i::Field, S, 11>,
        >,
    UnsaturatedContext<mike_fast_iii::Field, S, 16, 14, 17, 29>: PackedField<
            Scalar = mike_fast_iii::Field,
            Packed = UnsaturatedPacked<mike_fast_iii::Field, S, 17>,
        >,
    UnsaturatedContext<mike_fast_v::Field, S, 20, 19, 24, 27>: PackedField<
            Scalar = mike_fast_v::Field,
            Packed = UnsaturatedPacked<mike_fast_v::Field, S, 24>,
        >,
    UnsaturatedContext<mike_conservative_i::Field, S, 12, 11, 14, 28>: PackedField<
            Scalar = mike_conservative_i::Field,
            Packed = UnsaturatedPacked<mike_conservative_i::Field, S, 14>,
        >,
    UnsaturatedContext<mike_conservative_iii::Field, S, 18, 17, 20, 29>: PackedField<
            Scalar = mike_conservative_iii::Field,
            Packed = UnsaturatedPacked<mike_conservative_iii::Field, S, 20>,
        >,
    UnsaturatedContext<mike_conservative_v::Field, S, 24, 23, 27, 29>: PackedField<
            Scalar = mike_conservative_v::Field,
            Packed = UnsaturatedPacked<mike_conservative_v::Field, S, 27>,
        >,
{
    exercise::<mike_fast_i::Field, _, 10, 9, 11, 30>(simd);
    exercise::<mike_fast_iii::Field, _, 16, 14, 17, 29>(simd);
    exercise::<mike_fast_v::Field, _, 20, 19, 24, 27>(simd);
    exercise::<mike_conservative_i::Field, _, 12, 11, 14, 28>(simd);
    exercise::<mike_conservative_iii::Field, _, 18, 17, 20, 29>(simd);
    exercise::<mike_conservative_v::Field, _, 24, 23, 27, 29>(simd);
}

#[test]
fn arithmetic_for_every_supported_layout() {
    let level = fearless_simd::Level::new();
    #[cfg(target_arch = "aarch64")]
    if let Some(simd) = level.as_neon() {
        exercise_all_layouts(simd);
    }
    #[cfg(target_arch = "x86_64")]
    if let Some(simd) = level.as_avx2() {
        exercise_all_layouts(simd);
    }
}

struct Accepted(bool);

impl<F: Fq> PackedOperation<F> for Accepted {
    type Output = bool;

    fn run<C: PackedField<Scalar = F>>(self, _context: C) -> bool {
        self.0
    }
}

struct NotRun;

impl<F: Fq> PackedOperation<F> for NotRun {
    type Output = ();

    fn run<C: PackedField<Scalar = F>>(self, _context: C) {
        panic!("rejected dispatch consumed the operation");
    }
}

#[test]
fn dispatch_acceptance_and_rejection_are_explicit() {
    let level = fearless_simd::Level::new();
    let expected = {
        #[cfg(target_arch = "aarch64")]
        {
            level.as_neon().is_some()
        }
        #[cfg(target_arch = "x86_64")]
        {
            level.as_avx2().is_some()
        }
        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        {
            false
        }
    };
    assert_eq!(
        dispatch::<mike_fast_i::Field, _, 10, 9>(level, Accepted(true)).is_ok(),
        expected
    );
    assert!(dispatch::<mike_fast_i::Field, _, 8, 0>(level, NotRun).is_err());

    #[cfg(all(target_arch = "x86_64", feature = "test-simd-fallback"))]
    assert!(
        dispatch::<mike_fast_i::Field, _, 10, 9>(fearless_simd::Level::fallback(), NotRun,)
            .is_err()
    );
}

#[test]
fn constructor_checks_each_guard() {
    fearless_simd::dispatch!(fearless_simd::Level::new(), simd => {
        assert!(UnsaturatedContext::<mike_fast_i::Field, _, 10, 9, 11, 30>::new(simd).is_some());
        assert!(UnsaturatedContext::<mike_fast_i::Field, _, 8, 0, 11, 30>::new(simd).is_none(), "wrong scalar digit count");
        assert!(UnsaturatedContext::<mike_fast_i::Field, _, 10, 8, 11, 30>::new(simd).is_none(), "wrong all-ones prefix");
        assert!(UnsaturatedContext::<mike_fast_i::Field, _, 10, 9, 11, 27>::new(simd).is_none(), "insufficient unsaturated headroom");
        assert!(UnsaturatedContext::<incompatible_sparse_shape::Field, _, 10, 2, 11, 30>::new(simd).is_none(), "incompatible sparse modulus shape");
    });
}

fn expected_bits<const L: usize, const K: usize, const W: usize>(scalar: &[u32; L]) -> [u32; K] {
    core::array::from_fn(|limb| {
        let mut digit = 0u32;
        for bit in 0..W {
            let source = limb * W + bit;
            if source / 32 < L {
                digit |= ((scalar[source / 32] >> (source % 32)) & 1) << bit;
            }
        }
        digit
    })
}

fn expected_words<const L: usize, const K: usize, const W: usize>(digits: &[u32; K]) -> [u32; L] {
    core::array::from_fn(|word| {
        let mut value = 0u32;
        for bit in 0..32 {
            let source = word * 32 + bit;
            if source / W < K {
                value |= ((digits[source / W] >> (source % W)) & 1) << bit;
            }
        }
        value
    })
}

fn check_transcode_width<const L: usize, const K: usize, const W: usize>() {
    let scalar = core::array::from_fn(|word| {
        0x1020_3040u32
            .wrapping_mul((word + 1) as u32)
            .rotate_left((word * 7 % 32) as u32)
    });
    assert_eq!(
        scalar_to_unsaturated_digits::<L, K, W>(&scalar),
        expected_bits::<L, K, W>(&scalar)
    );

    fearless_simd::dispatch!(fearless_simd::Level::new(), simd => {
        let mask = (1u32 << W) - 1;
        let digits: [[u32; 4]; K] = core::array::from_fn(|limb| {
            core::array::from_fn(|lane| {
                0x3141_5926u32
                    .wrapping_mul((limb * 4 + lane + 1) as u32)
                    .rotate_left(((limb + lane) * 5 % 32) as u32)
                    & mask
            })
        });
        let packed = UnsaturatedPacked::<mike_fast_i::Field, _, K> {
            limbs: digits.map(|digit| u32x4::simd_from(simd, digit)),
            field: PhantomData,
        };
        let actual = unsaturated_to_scalar::<mike_fast_i::Field, _, L, K, W>(&packed);
        for lane in 0..4 {
            let input = core::array::from_fn(|limb| digits[limb][lane]);
            assert_eq!(
                core::array::from_fn::<_, L, _>(|word| actual[word][lane]),
                expected_words::<L, K, W>(&input),
            );
        }
    });
}

#[test]
fn conversion_matches_independent_bit_mapping() {
    check_transcode_width::<10, 11, 30>();
    check_transcode_width::<12, 14, 28>();
    check_transcode_width::<16, 17, 29>();
    check_transcode_width::<18, 20, 29>();
    check_transcode_width::<20, 24, 27>();
    check_transcode_width::<24, 27, 29>();
}
