//! Traits for implementing generic operations over finite fields.
//!
//! Currently includes two traits.
//!
//! - Fp: is a trait for generic finite field arithmetic over GF(p)
//! - Fp2: a supertrait of Fp for the finite field GF(p^2) with modulus x^2 + 1

pub mod fp;
pub mod fp2;

pub use fp::Fp;
pub use fp2::Fp2;
