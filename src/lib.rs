//! fp2-rs is a library for efficient and constant-time arithmetic for the finite
//! field Fp^2
//!
//! This library has been developed following a series of projects which needed
//! finite field arithmetic over a number of different characteristics.
//! Currently, the only intended usage for this code, is in isogeny-based
//! cryptographic research, meaning the current functionality is tailored for
//! a particular set of problems.

#![recursion_limit = "256"]

pub mod fp2_gen;
pub mod fp_gen;
pub mod test_macros;
pub mod traits;
pub mod utils64;
