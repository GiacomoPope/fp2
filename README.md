# Fp2

[![Build Status][build-image]][build-link]

An efficient, flexible and constant time Rust implementation of finite fields 
$\mathbb{F}\_{p}$ and $\mathbb{F}\_{p^2}$ where $p \equiv 3 \pmod 4$. Used currently for various Rust implementations of isogeny-based cryptographic protocols.

### Motivation

These two macros have ended up being stuck inside every rust crypto thing I've written recently for isogeny-based crypto. The idea of this repository is to dedicate a central place to work on them to avoid there being many related but incompatible versions throughout my projects.


## Usage

Fields can be defined using macros as follows:

```rs
// Fp251: a finite field element GF(p) with p = 3 mod 4. 
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
fp2_rs::define_fp_core!(
    typename = Fp251,
    words = 4_usize,
    bit_len = 251_usize,
    modulus = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x04FFFFFFFFFFFFFF],
    half_modulus = [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0280000000000000],
    mont_r = [0x0000000000000033, 0x0000000000000000, 0x0000000000000000, 0x0100000000000000],
    neg_r = [0xFFFFFFFFFFFFFFCC, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x03FFFFFFFFFFFFFF],
    two_r = [0x0000000000000066, 0x0000000000000000, 0x0000000000000000, 0x0200000000000000],
    three_r = [0x0000000000000099, 0x0000000000000000, 0x0000000000000000, 0x0300000000000000],
    four_r = [0x00000000000000CC, 0x0000000000000000, 0x0000000000000000, 0x0400000000000000],
    r_sqr = [0x3333333333333D70, 0x3333333333333333, 0x3333333333333333, 0x0333333333333333],
    minus_p_inv = 1_u64,
    div_correction = [0x49BA5E3BCD35A858, 0xF7CED916872B020C, 0x72B020C49BA5E353, 0x025E353F7CED9168],
    reduce_const = [0x3333333333333333, 0x3333333333333333, 0x3333333333333333, 0x0100000000000033],
    window_len = 5_usize,
    sqrt_el = 49_usize,
    sqrt_eh = [10],
    fourth_root_el = 49_usize,
    fourth_root_eh = [5],
    p1 = 2684354559_u64,
    p1_div_m = 11068046455220847252_u64,
);

// Fp251Ext: a finite field element GF(p^2) with modulus x^2 + 1. 
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
fp2_rs::define_fp2_core!(
    typename = Fp251Ext,
    base_field = Fp251,
    nqr_re = [0x0000000000000100, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]
);
```

The easiest way to generate macro parameters is to generate the above code snippets with the sage file [`scripts/gen_fp.sage`](scripts/gen_fp.sage).


### Tests

Tests can be run: 

```
cargo test --features test_macros
```

TODO: can we automatically enable the feature flag `test_macros` when running `cargo test`?

### Benchmarks

Benchmarks can be run with:

```
RUSTFLAGS="-C target-cpu=native" cargo bench
```

[//]: # (badges)

[build-image]: https://github.com/GiacomoPope/fp2-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/GiacomoPope/fp2-rs/actions?query=workflow%3ARust
