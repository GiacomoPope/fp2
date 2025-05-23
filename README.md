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
    typename = Fp2,
    base_typename = Fp,
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


### Tests

Tests can be run:

```
cargo test --features test_macros
```

### Benchmarks

Benchmarks can be run with:

```
RUSTFLAGS="-C target-cpu=native" cargo bench
```

[//]: # (badges)

[build-image]: https://github.com/GiacomoPope/fp2/workflows/Rust/badge.svg
[build-link]: https://github.com/GiacomoPope/fp2/actions?query=workflow%3ARust
