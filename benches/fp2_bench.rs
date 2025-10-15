mod bench_util;

macro_rules! define_fp_benchmarks {
    ($Fq:ty) => {
        fn benchmark_fp_mul(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);

            let bench_id = format!("Benchmarking x * y over Fp with {} bits", <$Fq>::BIT_LENGTH);
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x) * black_box(y)));
        }

        fn benchmark_sum_of_products(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

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
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking (sum of products) x * y over Fp2 with {} bits",
                <$Fq>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| {
                b.iter(|| black_box(x).mul_sum_of_products(black_box(y)))
            });
        }

        fn benchmark_school_fp2_mul(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fq>::rand(&mut rng);
            let y = <$Fq>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking (schoolbook) x * y over Fp2 with {} bits",
                <$Fq>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| {
                b.iter(|| black_box(x).mul_schoolbook(black_box(y)))
            });
        }

        criterion_group! {
            name = fp2_benchmarks;
            config = Criterion::default().measurement_time(Duration::from_secs(3));
            targets = benchmark_sop_fp2_mul, benchmark_school_fp2_mul
        }
    };
}

mod bench_251 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    static MODULUS: [u64; 4] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x04FFFFFFFFFFFFFF,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

mod bench_508 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    static MODULUS: [u64; 8] = [
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0x107FFFFFFFFFFFFFu64,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

mod bench_896 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    static MODULUS: [u64; 14] = [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xA7FFFFFFFFFFFFFF,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

mod bench_1008 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    static MODULUS: [u64; 16] = [
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0x0000EFFFFFFFFFFFu64,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

mod bench_1554 {
    use criterion::{Criterion, black_box, criterion_group, criterion_main};
    use std::time::Duration;

    static MODULUS: [u64; 25] = [
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0x0000000000047FFFu64,
    ];

    fp2::define_fp2_from_modulus!(typename = Fp2, base_typename = Fp, modulus = MODULUS,);

    define_fp_benchmarks!(Fp);
    define_fp2_benchmarks!(Fp2);

    criterion_main!(fp_benchmarks, fp2_benchmarks);
}

fn main() {
    bench_251::fp_benchmarks();
    bench_251::fp2_benchmarks();

    bench_508::fp_benchmarks();
    bench_508::fp2_benchmarks();

    bench_896::fp_benchmarks();
    bench_896::fp2_benchmarks();

    bench_1008::fp_benchmarks();
    bench_1008::fp2_benchmarks();

    bench_1554::fp_benchmarks();
    bench_1554::fp2_benchmarks();
}
