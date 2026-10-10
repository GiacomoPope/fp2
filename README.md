# Fp2

[![Build Status][build-image]][build-link]

An efficient, flexible and constant time Rust implementation of finite fields
$\mathbb{F}\_{p}$ and $\mathbb{F}\_{p^2}$ where $p \equiv 3 \pmod 4$. Used currently for various Rust implementations of isogeny-based cryptographic protocols.

## Motivation

These two macros have ended up being stuck inside every rust crypto thing I've written recently for isogeny-based crypto. The idea of this repository is to dedicate a central place to work on them to avoid there being many related but incompatible versions throughout my projects.


## Usage

The base field can be defined using the macro `define_fp_core` using the modulus as input:

```rs
// Fp251: a finite field element GF(p) with p = 3 mod 4.
// Contents are opaque, all functions are constant-time.
fp2::define_fp_core!(
    typename = Fp251,
    modulus = [0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64,
);
```

For the extension field, it can be generated directly from the modulus as with
the base field:

```rs
fp2::define_fp2_from_modulus!(
    typename = Fp251Ext,
    base_typename = Fp251,
    modulus = [0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64,
);
```

Or given a type for the base field the extension can be generated directly, which would allow users to supply their own GF(p) arithmetic to extend:

```rs
// Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1.
// Contents are opaque, all functions are constant-time.
fp2::define_fp2_from_type!(
    typename = Fp251Ext,
    base_field = Fp251,
);
```

The easiest way to generate macro parameters is to generate the above code snippets with the sage file [`scripts/gen_fp.sage`](scripts/gen_fp.sage).

### Packed arithmetic

`packed::PackedField` provides four-lane arithmetic. `Fq::with_packed` runs a
`PackedOperation` with the selected backend; its default uses scalar operations
and needs no additional field trait.

The optional `simd` feature accelerates supported generated prime fields;
unsupported fields and CPUs continue to use serial arithmetic. `simd` requires
Rust 1.89.

```rust
use fp2::packed::{PackedField, PackedOperation};
use fp2::traits::Fq;

struct Products<F>([F; 4], [F; 4]);
impl<F: Fq> PackedOperation<F> for Products<F> {
    type Output = [F; 4];
    fn run<C: PackedField<Scalar = F>>(self, ctx: C) -> Self::Output {
        ctx.vectorize(|| ctx.unpack(&ctx.mul(&ctx.pack(self.0), &ctx.pack(self.1))))
    }
}

fn products<F: Fq>(a: [F; 4], b: [F; 4]) -> [F; 4] {
    F::with_packed(Products(a, b))
}
```

The SIMD backend requires a low radix-32 modulus digit of `0xffffffff`, a nonzero
top digit, and `p < R/4`, where `R = 2^(64*N)`. Operations are constant-time with
respect to field values.

### Tests

Tests can be run:

```
cargo test --features test-utils
```

The test macros `define_fp_tests!` and `define_fp2_tests!` can also be used to test your own field types. They require `sha2` and `num-bigint` in your `[dev-dependencies]`.

### Benchmarks

Benchmarks can be run with:

```
RUSTFLAGS="-C target-cpu=native" cargo bench
```

[//]: # (badges)

[build-image]: https://github.com/GiacomoPope/fp2/workflows/Rust/badge.svg
[build-link]: https://github.com/GiacomoPope/fp2/actions?query=workflow%3ARust
