mod bench_util;

macro_rules! define_fp_benchmarks {
    ($Fq:ty) => {
        fn benchmark_fp_mul(c: &mut Criterion) {
            let mut rng = crate::bench_util::Drng::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);

            let bench_id = format!("Benchmarking x * y over Fp with {} bits", <$Fq>::BIT_LENGTH);
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x) * black_box(y)));
        }

        fn benchmark_sum_of_products(c: &mut Criterion) {
            let mut rng = crate::bench_util::Drng::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);
            let z = <$Fq>::rand(&mut rng);
            let w = <$Fq>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking a1*b1 + a2*b2 over Fp with {} bits",
                <$Fq>::BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| {
                b.iter(|| {
                    <$Fq>::sum_of_products(
                        &black_box(x),
                        &black_box(y),
                        &black_box(z),
                        &black_box(w),
                    )
                })
            });
        }

        criterion_group! {
            name = fp_benchmarks;
            config = Criterion::default().measurement_time(Duration::from_secs(3));
            targets = benchmark_fp_mul, benchmark_sum_of_products
        }
    };
}

macro_rules! define_fp2_benchmarks {
    ($Fq:ty) => {
        fn benchmark_sop_fp2_mul(c: &mut Criterion) {
            let mut rng = crate::bench_util::Drng::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking (sum of products) x * y over Fp2 with {} bits",
                <$Fq>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| {
                b.iter(|| black_box(x).mul_sum_of_products(&black_box(y)))
            });
        }

        fn benchmark_school_fp2_mul(c: &mut Criterion) {
            let mut rng = crate::bench_util::Drng::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking (schoolbook) x * y over Fp2 with {} bits",
                <$Fq>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| {
                b.iter(|| black_box(x).mul_schoolbook(&black_box(y)))
            });
        }

        criterion_group! {
            name = fp2_benchmarks;
            config = Criterion::default().measurement_time(Duration::from_secs(3));
            targets = benchmark_sop_fp2_mul, benchmark_school_fp2_mul
        }
    };
}

mod bench_p308_644 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    // p308.644
    static MODULUS: [u64; 5] = [
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0x278F_FFFF_FFFF_FFFF,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

mod bench_p474_593 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    // p474.593
    static MODULUS: [u64; 8] = [
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0x0000_0009_43FF_FFFF,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

mod bench_p628_317 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    // p628.317
    static MODULUS: [u64; 10] = [
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFF,
        0x13CF_FFFF_FFFF_FFFF,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

fn main() {
    bench_p308_644::fp_benchmarks();
    bench_p308_644::fp2_benchmarks();

    bench_p474_593::fp_benchmarks();
    bench_p474_593::fp2_benchmarks();

    bench_p628_317::fp_benchmarks();
    bench_p628_317::fp2_benchmarks();
}
