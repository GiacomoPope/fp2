//! Four-lane field arithmetic with serial and optional SIMD implementations.

use core::marker::PhantomData;

use crate::traits::Fq;

/// Operations on four independent field values.
///
/// Conversions preserve valid scalar values. Operations must be constant-time
/// with respect to field data; lane indices and other parameters are public.
pub trait PackedField: Copy {
    /// The scalar field represented by each lane.
    type Scalar: Fq;
    /// The backend's representation of four elements of [`Self::Scalar`].
    type Packed: Copy;

    /// Returns whether this context uses a hardware-accelerated backend.
    fn is_accelerated(&self) -> bool {
        false
    }

    /// Runs `f` once within the backend's vectorization boundary.
    ///
    /// Returns the result of `f`. The serial implementation calls `f` directly.
    fn vectorize<R>(&self, f: impl FnOnce() -> R) -> R {
        f()
    }

    /// Packs four scalar values in array order, preserving each field value.
    fn pack(&self, values: [Self::Scalar; 4]) -> Self::Packed;

    /// Returns the four scalar values in lane order.
    fn unpack(&self, value: &Self::Packed) -> [Self::Scalar; 4];

    /// Adds corresponding lanes of `a` and `b` in the scalar field.
    fn add(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed;

    /// Subtracts each lane of `b` from the corresponding lane of `a`.
    fn sub(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed;

    /// Multiplies corresponding lanes of `a` and `b` in the scalar field.
    fn mul(&self, a: &Self::Packed, b: &Self::Packed) -> Self::Packed;

    /// Squares each lane in the scalar field.
    fn square(&self, a: &Self::Packed) -> Self::Packed;

    /// Selects four lanes from concatenated packs.
    ///
    /// The result contains lanes `I0`, `I1`, `I2`, and `I3`, in that order.
    /// Index `i` selects lane `i % 4` of `values[i / 4]`; indices may repeat.
    ///
    /// # Panics
    ///
    /// Panics if any index is at least `4 * values.len()`.
    fn gather<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        &self,
        values: &[Self::Packed],
    ) -> Self::Packed;

    /// Computes the sum and difference of each lane pair.
    ///
    /// Each pair's lower lane receives the sum; its higher lane receives the
    /// lower-minus-higher difference. The lane-index XOR distance `BIT` is 1
    /// for pairs (0, 1) and (2, 3), or 2 for pairs (0, 2) and (1, 3).
    ///
    /// # Panics
    ///
    /// Panics if `BIT` is neither 1 nor 2.
    fn butterfly<const BIT: usize>(&self, value: &Self::Packed) -> Self::Packed;
}

/// A computation for a field's selected packed implementation.
///
/// A call to [`Fq::with_packed`] dispatches the computation once.
pub trait PackedOperation<F: Fq> {
    /// The backend-independent result of the computation.
    type Output;

    /// Executes the computation with the selected context.
    ///
    /// Calls to [`PackedField::vectorize`] provide a vectorization boundary for
    /// sequences of packed operations.
    fn run<C: PackedField<Scalar = F>>(self, context: C) -> Self::Output;
}

/// Four-lane arithmetic using ordinary scalar field operations.
#[derive(Clone, Copy)]
pub(crate) struct Serial<F: Fq>(PhantomData<F>);

impl<F: Fq> Serial<F> {
    pub(crate) const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<F: Fq> Default for Serial<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Fq> PackedField for Serial<F> {
    type Scalar = F;
    type Packed = [F; 4];

    #[inline(always)]
    fn pack(&self, values: [F; 4]) -> [F; 4] {
        values
    }
    #[inline(always)]
    fn unpack(&self, value: &[F; 4]) -> [F; 4] {
        *value
    }
    #[inline(always)]
    fn add(&self, a: &[F; 4], b: &[F; 4]) -> [F; 4] {
        core::array::from_fn(|i| a[i] + b[i])
    }
    #[inline(always)]
    fn sub(&self, a: &[F; 4], b: &[F; 4]) -> [F; 4] {
        core::array::from_fn(|i| a[i] - b[i])
    }
    #[inline(always)]
    fn mul(&self, a: &[F; 4], b: &[F; 4]) -> [F; 4] {
        core::array::from_fn(|i| a[i] * b[i])
    }
    #[inline(always)]
    fn square(&self, a: &[F; 4]) -> [F; 4] {
        a.map(Fq::square)
    }
    #[inline(always)]
    fn gather<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        &self,
        values: &[[F; 4]],
    ) -> [F; 4] {
        [I0, I1, I2, I3].map(|i| values[i / 4][i % 4])
    }
    #[inline(always)]
    fn butterfly<const BIT: usize>(&self, value: &[F; 4]) -> [F; 4] {
        assert!(BIT == 1 || BIT == 2);
        core::array::from_fn(|i| {
            if i & BIT == 0 {
                value[i] + value[i ^ BIT]
            } else {
                value[i ^ BIT] - value[i]
            }
        })
    }
}
