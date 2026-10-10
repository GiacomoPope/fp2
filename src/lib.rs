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
pub mod packed;
pub mod test_macros;
pub mod traits;
pub mod utils64;
pub use rand_core;

#[cfg(feature = "simd")]
mod simd;

/// Support for downstream expansions of this crate's macros.
#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "simd")]
    pub use crate::simd::{MontgomeryDigits, MontgomeryField, dispatch};

    /// Runs a generated field's packed computation with serial arithmetic.
    #[cfg(not(feature = "simd"))]
    #[inline]
    pub fn dispatch<F: crate::traits::Fq, O: crate::packed::PackedOperation<F>>(
        operation: O,
    ) -> O::Output {
        operation.run(crate::packed::Serial::<F>::new())
    }
}
