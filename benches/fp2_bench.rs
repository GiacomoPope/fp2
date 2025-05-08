mod bench_util;

macro_rules! define_fp2_benchmarks {
    ($Fp2:ty) => {
        use criterion::{black_box, criterion_group, criterion_main, Criterion};
        use std::time::Duration;

        fn benchmark_add(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);
            let y = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking x + y with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x) + black_box(y)));
        }

        fn benchmark_sub(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);
            let y = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking x - y with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x) - black_box(y)));
        }

        fn benchmark_mul(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);
            let y = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking x * y with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x) * black_box(y)));
        }

        fn benchmark_div(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);
            let y = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking x / y with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x) / black_box(y)));
        }

        fn benchmark_invert(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking 1 / x with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x).invert()));
        }

        fn benchmark_mul_new(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);
            let y = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking x * y (new method) with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x).mul_new(black_box(y))));
        }

        fn benchmark_mul_old(c: &mut Criterion) {
            let mut rng = crate::bench_util::DRNG::new();

            let x = <$Fp2>::rand(&mut rng);
            let y = <$Fp2>::rand(&mut rng);

            let bench_id = format!(
                "Benchmarking x * y (old method) with char(k) ~2^{}",
                <$Fp2>::CHAR_BIT_LENGTH
            );
            c.bench_function(&bench_id, |b| b.iter(|| black_box(x).mul_old(black_box(y))));
        }

        criterion_group! {
            name = fp2_benchmarks;
            config = Criterion::default().measurement_time(Duration::from_secs(3));
            targets = benchmark_add, benchmark_sub, benchmark_mul, benchmark_div, benchmark_invert, benchmark_mul_new, benchmark_mul_old
        }
    };
}
mod bench_fp_251_ext {
    // Fp251: a finite field element GF(p) with p = 3 mod 4.
    // Contents are opaque, all functions are constant-time.
    // Macro input generated with scripts/gen_fp.sage
    // p = 5*2^248 - 1
    fp2_rs::define_fp_core!(
        typename = Fp251,
        modulus = [
            0xFFFFFFFFFFFFFFFF_u64,
            0xFFFFFFFFFFFFFFFF_u64,
            0xFFFFFFFFFFFFFFFF_u64,
            0x04FFFFFFFFFFFFFF_u64
        ],
    );

    // Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1.
    // Contents are opaque, all functions are constant-time.
    // Macro input generated with scripts/gen_fp.sage
    fp2_rs::define_fp2_core!(typename = Fp251Ext, base_field = Fp251,);

    define_fp2_benchmarks!(Fp251Ext);
    criterion_main!(fp2_benchmarks);
}

fn main() {
    bench_fp_251_ext::fp2_benchmarks();
}
