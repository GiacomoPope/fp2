//! Four-way finite-field arithmetic over a `fearless_simd` backend.
//!
//! On AArch64 NEON and x86-64 AVX2, supported sparse moduli use an unsaturated Montgomery radix,
//! with conversion at pack/unpack boundaries. Unsupported configurations use
//! the radix-32 packed context, and unavailable SIMD levels use serial execution.

use core::marker::PhantomData;

use crate::packed::{PackedOperation, Serial};
use crate::traits::Fq;

mod radix32;

mod unsaturated;

#[cfg(test)]
mod tests;

/// Runs a generated field's packed computation with SIMD when supported.
///
/// The radix-32 limb count is `L`; a nonzero `LOW` specifies the all-ones prefix length.
#[doc(hidden)]
#[inline]
pub fn dispatch<F: MontgomeryField, O: PackedOperation<F>, const L: usize, const LOW: usize>(
    operation: O,
) -> O::Output {
    dispatch_at::<F, O, L, LOW>(fearless_simd::Level::new(), operation)
}

/// Runs a packed computation at the given SIMD level, with serial fallback.
///
/// The unsaturated backend gets first choice on its supported NEON and AVX2 layouts.
/// All other levels and configurations use the radix-32 backend when available,
/// with serial arithmetic as the final fallback.
fn dispatch_at<F: MontgomeryField, O: PackedOperation<F>, const L: usize, const LOW: usize>(
    level: fearless_simd::Level,
    operation: O,
) -> O::Output {
    let operation = match unsaturated::dispatch::<F, O, L, LOW>(level, operation) {
        Ok(output) => return output,
        Err(operation) => operation,
    };
    if level.is_fallback() {
        return operation.run(Serial::<F>::new());
    }
    fearless_simd::dispatch!(level, simd => {
        match radix32::Context::<F, _, L, LOW>::new(simd) {
            Some(context) => operation.run(context),
            None => operation.run(Serial::<F>::new()),
        }
    })
}

/// The Montgomery conversion bridge for [`crate::define_fp_core`].
///
/// [`Self::MODULUS`] contains little-endian radix-2^64 limbs with no leading
/// zero limb. Conversions preserve the reduced Montgomery integer with
/// `R = 2^(64*N)`, using `2*N` little-endian radix-32 digits, where
/// `N = MODULUS.len()`. Both conversions must be constant-time with respect to
/// field values.
#[doc(hidden)]
pub trait MontgomeryField: Fq {
    /// The field characteristic in little-endian radix-2^64 limbs.
    const MODULUS: &'static [u64];

    /// Writes this value's reduced Montgomery representation as radix-32 digits.
    ///
    /// The digits are written in little-endian order.
    ///
    /// # Panics
    ///
    /// Panics if `digits.len()` is not `2 * Self::MODULUS.len()`.
    fn write_montgomery_digits(self, digits: &mut [u32]);

    /// Reconstructs a field value from canonical Montgomery digits without rescaling.
    fn from_montgomery_digits(digits: MontgomeryDigits<'_, Self>) -> Self;
}

/// Canonical radix-2^32 Montgomery digits for `F`, produced by this backend.
#[doc(hidden)]
pub struct MontgomeryDigits<'a, F: Fq> {
    digits: &'a [u32],
    field: PhantomData<F>,
}

impl<'a, F: Fq> MontgomeryDigits<'a, F> {
    /// Borrows the little-endian radix-32 digits.
    #[doc(hidden)]
    pub fn as_slice(&self) -> &'a [u32] {
        self.digits
    }
}
