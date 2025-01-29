# Fp2

These two macros have ended up being stuck inside every rust crypto thing I've written recently and they're messy and always kind of different. The idea of this repo is to formalise these a little better and make them "rusty".

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
