//! fp2-rs is a library for efficient and constant-time arithmetic for the finite
//! field Fp^2
//!
//! This library has been developed following a series of projects which needed
//! finite field arithmetic over a number of different characteristics.
//! Currently, the only intended usage for this code, is in isogeny-based
//! cryptographic research, meaning the current functionality is tailored for
//! a particular set of problems.

// We include these so we can have things like
// fn encode(self) -> [u8; Self::ENCODED_LENGTH];
// defined within the Fq trait
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod fp2_gen;
pub mod fp_gen;
pub mod test_macros;
pub mod traits;
pub mod utils64;
