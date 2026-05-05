use core::ops::{
    Add, AddAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
#[cfg(feature = "panicking-ops")]
use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::{
    common::{Inner, SaturatingFrom},
    si::{Si8, Si16, Si32, Si64, Si128, Sisize},
    su::{Su8, Su16, Su32, Su64, Su128, Susize},
};

/// Error returned by fallible division and remainder operations.
///
/// # Examples
///
/// ```
/// use satint::{DivError, TryDiv, si8};
///
/// assert_eq!(si8(10).try_div(0_i8), Err(DivError::DivisionByZero));
/// ```
#[expect(
    clippy::exhaustive_enums,
    reason = "division failures are limited to zero divisors and primitive overflow"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivError {
    /// The divisor was zero.
    DivisionByZero,
    /// The operation overflowed. Only reachable for signed types, where
    /// `MIN / -1` and `MIN % -1` exceed the representable range.
    Overflow,
}

impl core::fmt::Display for DivError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::DivisionByZero => f.write_str("division by zero"),
            Self::Overflow => f.write_str("arithmetic overflow"),
        }
    }
}

impl core::error::Error for DivError {}

/// MSRV-compatible substitute for `u128::saturating_sub_signed`, which is
/// stable since 1.90 — newer than this crate's minimum supported version.
#[inline]
const fn u128_saturating_sub_signed(lhs: u128, rhs: i128) -> u128 {
    if rhs >= 0 {
        lhs.saturating_sub(rhs as u128)
    } else {
        lhs.saturating_add(rhs.unsigned_abs())
    }
}

/// Fallible division that distinguishes division by zero from overflow.
///
/// # Examples
///
/// ```
/// use satint::{DivError, TryDiv, si8, si16};
///
/// assert_eq!(si8(10).try_div(si16(2)), Ok(si8(5)));
/// assert_eq!(si8(10).try_div(0_i8), Err(DivError::DivisionByZero));
/// ```
pub trait TryDiv<Rhs = Self> {
    /// Result type produced by the division.
    type Output;

    /// Divides `self` by `rhs`.
    ///
    /// # Errors
    ///
    /// Returns [`DivError::DivisionByZero`] when `rhs` is zero and
    /// [`DivError::Overflow`] when the primitive operation overflows.
    ///
    /// # Examples
    ///
    /// ```
    /// use satint::{TryDiv, si8};
    ///
    /// assert_eq!(si8(12).try_div(3_i8), Ok(si8(4)));
    /// ```
    fn try_div(self, rhs: Rhs) -> Result<Self::Output, DivError>;
}

/// Fallible remainder that distinguishes division by zero from overflow.
///
/// # Examples
///
/// ```
/// use satint::{DivError, TryRem, si8};
///
/// assert_eq!(si8(10).try_rem(3_i8), Ok(si8(1)));
/// assert_eq!(si8(10).try_rem(0_i8), Err(DivError::DivisionByZero));
/// ```
pub trait TryRem<Rhs = Self> {
    /// Result type produced by the remainder operation.
    type Output;

    /// Calculates `self % rhs`.
    ///
    /// # Errors
    ///
    /// Returns [`DivError::DivisionByZero`] when `rhs` is zero and
    /// [`DivError::Overflow`] when the primitive operation overflows.
    ///
    /// # Examples
    ///
    /// ```
    /// use satint::{TryRem, si8};
    ///
    /// assert_eq!(si8(12).try_rem(5_i8), Ok(si8(2)));
    /// ```
    fn try_rem(self, rhs: Rhs) -> Result<Self::Output, DivError>;
}

/// Fallible division assignment.
///
/// # Examples
///
/// ```
/// use satint::{TryDivAssign, si8};
///
/// let mut value = si8(12);
/// value.try_div_assign(3_i8).unwrap();
/// assert_eq!(value, si8(4));
/// ```
pub trait TryDivAssign<Rhs = Self> {
    /// Divides `self` by `rhs` in place.
    ///
    /// # Errors
    ///
    /// Returns [`DivError::DivisionByZero`] when `rhs` is zero and
    /// [`DivError::Overflow`] when the primitive operation overflows.
    ///
    /// Leaves `self` unchanged if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use satint::{DivError, TryDivAssign, si8};
    ///
    /// let mut value = si8(12);
    /// assert_eq!(value.try_div_assign(0_i8), Err(DivError::DivisionByZero));
    /// assert_eq!(value, si8(12));
    /// ```
    fn try_div_assign(&mut self, rhs: Rhs) -> Result<(), DivError>;
}

/// Fallible remainder assignment.
///
/// # Examples
///
/// ```
/// use satint::{TryRemAssign, si8};
///
/// let mut value = si8(12);
/// value.try_rem_assign(5_i8).unwrap();
/// assert_eq!(value, si8(2));
/// ```
pub trait TryRemAssign<Rhs = Self> {
    /// Calculates `*self %= rhs` in place.
    ///
    /// # Errors
    ///
    /// Returns [`DivError::DivisionByZero`] when `rhs` is zero and
    /// [`DivError::Overflow`] when the primitive operation overflows.
    ///
    /// Leaves `self` unchanged if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use satint::{DivError, TryRemAssign, si8};
    ///
    /// let mut value = si8(12);
    /// assert_eq!(value.try_rem_assign(0_i8), Err(DivError::DivisionByZero));
    /// assert_eq!(value, si8(12));
    /// ```
    fn try_rem_assign(&mut self, rhs: Rhs) -> Result<(), DivError>;
}

macro_rules! generate_wrapper_to_wrapper_ops {
    ($lhs:ident; $as_ty:ty; $($rhs:ident),+ $(,)?) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = Self;

                #[inline]
                fn add(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Self::saturating_from(lhs.saturating_add(rhs))
                }
            }

            impl AddAssign<$rhs> for $lhs {
                #[inline]
                fn add_assign(&mut self, rhs: $rhs) {
                    *self = *self + rhs;
                }
            }

            impl Mul<$rhs> for $lhs {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Self::saturating_from(lhs.saturating_mul(rhs))
                }
            }

            impl MulAssign<$rhs> for $lhs {
                #[inline]
                fn mul_assign(&mut self, rhs: $rhs) {
                    *self = *self * rhs;
                }
            }

            impl Sub<$rhs> for $lhs {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Self::saturating_from(lhs.saturating_sub(rhs))
                }
            }

            impl SubAssign<$rhs> for $lhs {
                #[inline]
                fn sub_assign(&mut self, rhs: $rhs) {
                    *self = *self - rhs;
                }
            }

            impl TryDiv<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn try_div(self, rhs: $rhs) -> Result<Self::Output, DivError> {
                    if rhs == $rhs::ZERO {
                        return Err(DivError::DivisionByZero);
                    }

                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Ok(Self::saturating_from(lhs.saturating_div(rhs)))
                }
            }

            impl TryDivAssign<$rhs> for $lhs {
                #[inline]
                fn try_div_assign(&mut self, rhs: $rhs) -> Result<(), DivError> {
                    *self = (*self).try_div(rhs)?;
                    Ok(())
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl Div<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn div(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Self::saturating_from(lhs.saturating_div(rhs))
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl DivAssign<$rhs> for $lhs {
                #[inline]
                fn div_assign(&mut self, rhs: $rhs) {
                    *self = (*self).div(rhs);
                }
            }

            impl TryRem<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn try_rem(self, rhs: $rhs) -> Result<Self::Output, DivError> {
                    if rhs == $rhs::ZERO {
                        return Err(DivError::DivisionByZero);
                    }

                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Ok(Self::saturating_from(lhs.checked_rem(rhs).ok_or(DivError::Overflow)?))
                }
            }

            impl TryRemAssign<$rhs> for $lhs {
                #[inline]
                fn try_rem_assign(&mut self, rhs: $rhs) -> Result<(), DivError> {
                    *self = (*self).try_rem(rhs)?;
                    Ok(())
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl Rem<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn rem(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs.into_inner() as $as_ty;
                    Self::saturating_from(lhs % rhs)
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl RemAssign<$rhs> for $lhs {
                #[inline]
                fn rem_assign(&mut self, rhs: $rhs) {
                    *self = (*self).rem(rhs);
                }
            }
        )+
    };
}

macro_rules! generate_primitive_to_wrapper_ops {
    ($lhs:ident; $as_ty:ty; $($rhs:ident),+ $(,)?) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = Self;

                #[inline]
                fn add(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Self::saturating_from(lhs.saturating_add(rhs))
                }
            }

            impl AddAssign<$rhs> for $lhs {
                #[inline]
                fn add_assign(&mut self, rhs: $rhs) {
                    *self = *self + rhs;
                }
            }

            impl Mul<$rhs> for $lhs {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Self::saturating_from(lhs.saturating_mul(rhs))
                }
            }

            impl MulAssign<$rhs> for $lhs {
                #[inline]
                fn mul_assign(&mut self, rhs: $rhs) {
                    *self = *self * rhs;
                }
            }

            impl Sub<$rhs> for $lhs {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Self::saturating_from(lhs.saturating_sub(rhs))
                }
            }

            impl SubAssign<$rhs> for $lhs {
                #[inline]
                fn sub_assign(&mut self, rhs: $rhs) {
                    *self = *self - rhs;
                }
            }

            impl TryDiv<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn try_div(self, rhs: $rhs) -> Result<Self::Output, DivError> {
                    if rhs == 0 {
                        return Err(DivError::DivisionByZero);
                    }

                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Ok(Self::saturating_from(lhs.saturating_div(rhs)))
                }
            }

            impl TryDivAssign<$rhs> for $lhs {
                #[inline]
                fn try_div_assign(&mut self, rhs: $rhs) -> Result<(), DivError> {
                    *self = (*self).try_div(rhs)?;
                    Ok(())
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl Div<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn div(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Self::saturating_from(lhs.saturating_div(rhs))
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl DivAssign<$rhs> for $lhs {
                #[inline]
                fn div_assign(&mut self, rhs: $rhs) {
                    *self = (*self).div(rhs);
                }
            }

            impl TryRem<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn try_rem(self, rhs: $rhs) -> Result<Self::Output, DivError> {
                    if rhs == 0 {
                        return Err(DivError::DivisionByZero);
                    }

                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Ok(Self::saturating_from(lhs.checked_rem(rhs).ok_or(DivError::Overflow)?))
                }
            }

            impl TryRemAssign<$rhs> for $lhs {
                #[inline]
                fn try_rem_assign(&mut self, rhs: $rhs) -> Result<(), DivError> {
                    *self = (*self).try_rem(rhs)?;
                    Ok(())
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl Rem<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn rem(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as $as_ty;
                    let rhs = rhs as $as_ty;
                    Self::saturating_from(lhs % rhs)
                }
            }

            #[cfg(feature = "panicking-ops")]
            impl RemAssign<$rhs> for $lhs {
                #[inline]
                fn rem_assign(&mut self, rhs: $rhs) {
                    *self = (*self).rem(rhs);
                }
            }
        )+
    };
}

macro_rules! generate_wrapper_to_wrapper_signed_to_unsigned {
    ($lhs:ident; $($rhs:ident),+ $(,)?) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn add(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as i128;
                    let rhs = rhs.into_inner() as u128;
                    Self::saturating_from(lhs.saturating_add_unsigned(rhs))
                }
            }

            impl AddAssign<$rhs> for $lhs {
                #[inline]
                fn add_assign(&mut self, rhs: $rhs) {
                    *self = *self + rhs;
                }
            }

            impl Sub<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn sub(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as i128;
                    let rhs = rhs.into_inner() as u128;
                    Self::saturating_from(lhs.saturating_sub_unsigned(rhs))
                }
            }

            impl SubAssign<$rhs> for $lhs {
                #[inline]
                fn sub_assign(&mut self, rhs: $rhs) {
                    *self = *self - rhs;
                }
            }
        )+
    };
}

macro_rules! generate_wrapper_to_wrapper_unsigned_to_signed {
    ($lhs:ident; $($rhs:ident),+ $(,)?) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn add(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as u128;
                    let rhs = rhs.into_inner() as i128;
                    Self::saturating_from(lhs.saturating_add_signed(rhs))
                }
            }

            impl AddAssign<$rhs> for $lhs {
                #[inline]
                fn add_assign(&mut self, rhs: $rhs) {
                    *self = *self + rhs;
                }
            }

            impl Sub<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn sub(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as u128;
                    let rhs = rhs.into_inner() as i128;
                    Self::saturating_from(u128_saturating_sub_signed(lhs, rhs))
                }
            }

            impl SubAssign<$rhs> for $lhs {
                #[inline]
                fn sub_assign(&mut self, rhs: $rhs) {
                    *self = *self - rhs;
                }
            }
        )+
    };
}

macro_rules! generate_primitive_to_wrapper_signed_to_unsigned {
    ($lhs:ident; $($rhs:ident),+ $(,)?) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn add(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as i128;
                    let rhs = rhs as u128;
                    Self::saturating_from(lhs.saturating_add_unsigned(rhs))
                }
            }

            impl AddAssign<$rhs> for $lhs {
                #[inline]
                fn add_assign(&mut self, rhs: $rhs) {
                    *self = *self + rhs;
                }
            }

            impl Sub<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn sub(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as i128;
                    let rhs = rhs as u128;
                    Self::saturating_from(lhs.saturating_sub_unsigned(rhs))
                }
            }

            impl SubAssign<$rhs> for $lhs {
                #[inline]
                fn sub_assign(&mut self, rhs: $rhs) {
                    *self = *self - rhs;
                }
            }
        )+
    };
}

macro_rules! generate_primitive_to_wrapper_unsigned_to_signed {
    ($lhs:ident; $($rhs:ident),+ $(,)?) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn add(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as u128;
                    let rhs = rhs as i128;
                    Self::saturating_from(lhs.saturating_add_signed(rhs))
                }
            }

            impl AddAssign<$rhs> for $lhs {
                #[inline]
                fn add_assign(&mut self, rhs: $rhs) {
                    *self = *self + rhs;
                }
            }

            impl Sub<$rhs> for $lhs {
                type Output = $lhs;

                #[inline]
                fn sub(self, rhs: $rhs) -> Self::Output {
                    let lhs = self.into_inner() as u128;
                    let rhs = rhs as i128;
                    Self::saturating_from(u128_saturating_sub_signed(lhs, rhs))
                }
            }

            impl SubAssign<$rhs> for $lhs {
                #[inline]
                fn sub_assign(&mut self, rhs: $rhs) {
                    *self = *self - rhs;
                }
            }
        )+
    };
}

macro_rules! generate_signed_ops {
    ($($name:ident),+ $(,)?) => {
        $(
            impl Shl<u32> for $name {
                type Output = Self;

                #[inline]
                fn shl(self, rhs: u32) -> Self {
                    type InnerTy = <$name as Inner>::Inner;

                    let value = self.into_inner();

                    if value == 0 || rhs == 0 {
                        return self;
                    }

                    if rhs >= InnerTy::BITS {
                        return if value > 0 {
                            Self::MAX
                        } else {
                            Self::MIN
                        };
                    }

                    let headroom = if value >= 0 {
                        value.leading_zeros()
                    } else {
                        value.leading_ones()
                    };

                    if headroom > rhs {
                        Self::new(value << rhs)
                    } else if value > 0 {
                        Self::MAX
                    } else {
                        Self::MIN
                    }
                }
            }

            impl ShlAssign<u32> for $name {
                #[inline]
                fn shl_assign(&mut self, rhs: u32) {
                    *self = *self << rhs;
                }
            }

            impl Shr<u32> for $name {
                type Output = Self;

                #[inline]
                fn shr(self, rhs: u32) -> Self {
                    type InnerTy = <$name as Inner>::Inner;

                    let value = self.into_inner();
                    Self::new(if rhs >= InnerTy::BITS {
                        value >> (InnerTy::BITS - 1)
                    } else {
                        value >> rhs
                    })
                }
            }

            impl ShrAssign<u32> for $name {
                #[inline]
                fn shr_assign(&mut self, rhs: u32) {
                    *self = *self >> rhs;
                }
            }

            impl Neg for $name {
                type Output = Self;

                #[inline]
                fn neg(self) -> Self::Output {
                    Self::new(self.into_inner().saturating_neg())
                }
            }

            generate_wrapper_to_wrapper_ops!($name; i128; Si8, Si16, Si32, Si64, Si128, Sisize);
            generate_primitive_to_wrapper_ops!($name; i128; i8, i16, i32, i64, i128, isize);
            generate_wrapper_to_wrapper_signed_to_unsigned!($name; Su8, Su16, Su32, Su64, Su128, Susize);
            generate_primitive_to_wrapper_signed_to_unsigned!($name; u8, u16, u32, u64, u128, usize);
        )+
    };
}

generate_signed_ops!(Si8, Si16, Si32, Si64, Si128, Sisize);

macro_rules! generate_unsigned_ops {
    ($($name:ident),+ $(,)?) => {
        $(
            impl Shl<u32> for $name {
                type Output = Self;

                #[inline]
                fn shl(self, rhs: u32) -> Self {
                    type InnerTy = <$name as Inner>::Inner;

                    let value = self.into_inner();

                    if rhs >= InnerTy::BITS || value.leading_zeros() < rhs {
                        Self::MAX
                    } else {
                        Self::new(value << rhs) // safe: rhs < BITS and no high bits set
                    }
                }
            }

            impl ShlAssign<u32> for $name {
                #[inline]
                fn shl_assign(&mut self, rhs: u32) {
                    *self = *self << rhs;
                }
            }

            impl Shr<u32> for $name {
                type Output = Self;

                #[inline]
                fn shr(self, rhs: u32) -> Self {
                    type InnerTy = <$name as Inner>::Inner;

                    let value = self.into_inner();

                    Self::new(if rhs >= InnerTy::BITS {
                        0
                    } else {
                        value >> rhs
                    })
                }
            }

            impl ShrAssign<u32> for $name {
                #[inline]
                fn shr_assign(&mut self, rhs: u32) {
                    *self = *self >> rhs;
                }
            }

            generate_wrapper_to_wrapper_ops!($name; u128; Su8, Su16, Su32, Su64, Su128, Susize);
            generate_primitive_to_wrapper_ops!($name; u128; u8, u16, u32, u64, u128, usize);
            generate_wrapper_to_wrapper_unsigned_to_signed!($name; Si8, Si16, Si32, Si64, Si128, Sisize);
            generate_primitive_to_wrapper_unsigned_to_signed!($name; i8, i16, i32, i64, i128, isize);
        )+
    };
}

generate_unsigned_ops!(Su8, Su16, Su32, Su64, Su128, Susize);

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        ops::{DivError, TryDiv, TryDivAssign, TryRem, TryRemAssign},
        si::{Si8, Si16, Si128},
        su::{Su8, Su16, Su128},
    };

    #[test]
    fn div_error_display_and_traits() {
        assert_eq!(format!("{}", DivError::DivisionByZero), "division by zero");
        assert_eq!(format!("{}", DivError::Overflow), "arithmetic overflow");
        // Debug, Clone, Copy, PartialEq.
        let e = DivError::Overflow;
        let copied = e;
        assert_eq!(e, copied);
        assert_eq!(e.clone(), DivError::Overflow);
        assert_ne!(DivError::Overflow, DivError::DivisionByZero);
        // Error trait is implemented.
        let _: &dyn core::error::Error = &DivError::Overflow;
    }

    #[test]
    fn wrapper_to_wrapper_arith_signed() {
        // Add: normal and saturating.
        assert_eq!(Si8::new(1) + Si8::new(2), Si8::new(3));
        assert_eq!(Si8::MAX + Si8::new(1), Si8::MAX);
        assert_eq!(Si8::MIN + Si8::new(-1), Si8::MIN);
        // Sub.
        assert_eq!(Si8::new(5) - Si8::new(3), Si8::new(2));
        assert_eq!(Si8::MIN - Si8::new(1), Si8::MIN);
        // Mul.
        assert_eq!(Si8::new(3) * Si8::new(4), Si8::new(12));
        assert_eq!(Si8::MAX * Si8::new(2), Si8::MAX);
        assert_eq!(Si8::MIN * Si8::new(2), Si8::MIN);
        // Assign variants.
        let mut x = Si8::new(5);
        x += Si8::new(3);
        assert_eq!(x, Si8::new(8));
        x -= Si8::new(2);
        assert_eq!(x, Si8::new(6));
        x *= Si8::new(2);
        assert_eq!(x, Si8::new(12));
    }

    #[test]
    fn wrapper_to_wrapper_arith_unsigned() {
        assert_eq!(Su8::new(1) + Su8::new(2), Su8::new(3));
        assert_eq!(Su8::MAX + Su8::new(1), Su8::MAX);
        assert_eq!(Su8::new(5) - Su8::new(3), Su8::new(2));
        assert_eq!(Su8::ZERO - Su8::new(1), Su8::ZERO);
        assert_eq!(Su8::new(3) * Su8::new(4), Su8::new(12));
        assert_eq!(Su8::MAX * Su8::new(2), Su8::MAX);

        let mut x = Su8::new(5);
        x += Su8::new(3);
        assert_eq!(x, Su8::new(8));
        x -= Su8::new(2);
        assert_eq!(x, Su8::new(6));
        x *= Su8::new(2);
        assert_eq!(x, Su8::new(12));
    }

    #[test]
    fn try_div_and_rem_wrapper() {
        assert_eq!(Si8::new(10).try_div(Si8::new(3)), Ok(Si8::new(3)));
        assert_eq!(
            Si8::new(10).try_div(Si8::ZERO),
            Err(DivError::DivisionByZero),
        );
        assert_eq!(Si8::new(10).try_rem(Si8::new(3)), Ok(Si8::new(1)));
        assert_eq!(
            Si8::new(10).try_rem(Si8::ZERO),
            Err(DivError::DivisionByZero),
        );
        // saturating_div on i128 cannot overflow for Si8 inputs (widened), so
        // try_div over i128 won't surface Overflow; try_rem uses checked_rem.
        // For the in-band overflow path, Si128::MIN % Si128::new(-1) reaches it.
        assert_eq!(Si128::MIN.try_rem(Si128::new(-1)), Err(DivError::Overflow),);

        // Assign variants: success leaves new value; failure leaves it unchanged.
        let mut div_acc = Si8::new(10);
        assert_eq!(div_acc.try_div_assign(Si8::new(3)), Ok(()));
        assert_eq!(div_acc, Si8::new(3));
        let div_err = div_acc.try_div_assign(Si8::ZERO);
        assert_eq!(div_err, Err(DivError::DivisionByZero));
        assert_eq!(div_acc, Si8::new(3));

        let mut rem_acc = Si8::new(10);
        assert_eq!(rem_acc.try_rem_assign(Si8::new(3)), Ok(()));
        assert_eq!(rem_acc, Si8::new(1));
        let rem_err = rem_acc.try_rem_assign(Si8::ZERO);
        assert_eq!(rem_err, Err(DivError::DivisionByZero));
        assert_eq!(rem_acc, Si8::new(1));
    }

    #[cfg(feature = "panicking-ops")]
    #[test]
    fn panicking_div_rem_wrapper() {
        use core::ops::{Div, DivAssign, Rem, RemAssign};
        assert_eq!(Si8::new(10).div(Si8::new(3)), Si8::new(3));
        assert_eq!(Si8::new(10).rem(Si8::new(3)), Si8::new(1));
        let mut div_acc = Si8::new(10);
        div_acc.div_assign(Si8::new(3));
        assert_eq!(div_acc, Si8::new(3));
        let mut rem_acc = Si8::new(10);
        rem_acc.rem_assign(Si8::new(3));
        assert_eq!(rem_acc, Si8::new(1));
    }

    #[test]
    fn primitive_to_wrapper_arith() {
        // Signed wrapper with signed primitive.
        assert_eq!(Si8::new(1) + 2_i32, Si8::new(3));
        assert_eq!(Si8::MAX + 1_i32, Si8::MAX);
        assert_eq!(Si8::new(5) - 3_i32, Si8::new(2));
        assert_eq!(Si8::new(3) * 4_i32, Si8::new(12));

        let mut signed_acc = Si8::new(5);
        signed_acc += 3_i32;
        assert_eq!(signed_acc, Si8::new(8));
        signed_acc -= 1_i32;
        assert_eq!(signed_acc, Si8::new(7));
        signed_acc *= 2_i32;
        assert_eq!(signed_acc, Si8::new(14));

        // Unsigned wrapper with unsigned primitive.
        assert_eq!(Su8::new(1) + 2_u32, Su8::new(3));
        assert_eq!(Su8::new(5) - 3_u32, Su8::new(2));
        assert_eq!(Su8::new(3) * 4_u32, Su8::new(12));
        let mut y = Su8::new(5);
        y += 3_u32;
        y -= 1_u32;
        y *= 2_u32;
        assert_eq!(y, Su8::new(14));

        // try_div and try_rem with primitive RHS.
        assert_eq!(Si8::new(10).try_div(3_i32), Ok(Si8::new(3)));
        assert_eq!(Si8::new(10).try_div(0_i32), Err(DivError::DivisionByZero));
        assert_eq!(Si8::new(10).try_rem(3_i32), Ok(Si8::new(1)));
        assert_eq!(Si8::new(10).try_rem(0_i32), Err(DivError::DivisionByZero));
        // Assign variants.
        let mut div_acc = Si8::new(10);
        assert_eq!(div_acc.try_div_assign(3_i32), Ok(()));
        assert_eq!(div_acc, Si8::new(3));
        let mut rem_acc = Si8::new(10);
        assert_eq!(rem_acc.try_rem_assign(3_i32), Ok(()));
        assert_eq!(rem_acc, Si8::new(1));
    }

    #[cfg(feature = "panicking-ops")]
    #[test]
    fn panicking_div_rem_primitive() {
        use core::ops::{Div, DivAssign, Rem, RemAssign};
        assert_eq!(Si8::new(10).div(3_i32), Si8::new(3));
        assert_eq!(Si8::new(10).rem(3_i32), Si8::new(1));
        let mut div_acc = Si8::new(10);
        div_acc.div_assign(3_i32);
        assert_eq!(div_acc, Si8::new(3));
        let mut rem_acc = Si8::new(10);
        rem_acc.rem_assign(3_i32);
        assert_eq!(rem_acc, Si8::new(1));
    }

    #[test]
    fn cross_sign_wrapper_arith() {
        // Signed wrapper +/- unsigned wrapper.
        assert_eq!(Si8::new(1) + Su8::new(2), Si8::new(3));
        assert_eq!(Si8::MAX + Su8::new(10), Si8::MAX);
        assert_eq!(Si8::new(5) - Su8::new(3), Si8::new(2));
        assert_eq!(Si8::MIN - Su8::new(10), Si8::MIN);
        let mut x = Si8::new(5);
        x += Su8::new(3);
        assert_eq!(x, Si8::new(8));
        x -= Su8::new(2);
        assert_eq!(x, Si8::new(6));

        // Unsigned wrapper +/- signed wrapper.
        assert_eq!(Su8::new(5) + Si8::new(3), Su8::new(8));
        assert_eq!(Su8::new(1) + Si8::new(-1), Su8::ZERO);
        assert_eq!(Su8::MAX + Si8::new(10), Su8::MAX);
        assert_eq!(Su8::new(5) - Si8::new(3), Su8::new(2));
        assert_eq!(Su8::new(5) - Si8::new(-3), Su8::new(8));
        let mut y = Su8::new(5);
        y += Si8::new(2);
        assert_eq!(y, Su8::new(7));
        y -= Si8::new(1);
        assert_eq!(y, Su8::new(6));
    }

    #[test]
    fn cross_sign_primitive_arith() {
        // Signed wrapper +/- unsigned primitive.
        assert_eq!(Si8::new(1) + 2_u32, Si8::new(3));
        assert_eq!(Si8::MAX + 100_u32, Si8::MAX);
        assert_eq!(Si8::new(5) - 3_u32, Si8::new(2));
        assert_eq!(Si8::MIN - 100_u32, Si8::MIN);
        let mut x = Si8::new(5);
        x += 3_u32;
        assert_eq!(x, Si8::new(8));
        x -= 2_u32;
        assert_eq!(x, Si8::new(6));

        // Unsigned wrapper +/- signed primitive.
        assert_eq!(Su8::new(5) + 3_i32, Su8::new(8));
        assert_eq!(Su8::new(5) - 3_i32, Su8::new(2));
        let mut y = Su8::new(5);
        y += 3_i32;
        assert_eq!(y, Su8::new(8));
        y -= 2_i32;
        assert_eq!(y, Su8::new(6));
    }

    #[test]
    fn signed_shifts_and_neg() {
        // Shl: trivial branches (value == 0 or rhs == 0 returns self).
        assert_eq!(Si8::ZERO << 3, Si8::ZERO);
        assert_eq!(Si8::new(5) << 0, Si8::new(5));
        // Shl: rhs >= BITS for positive saturates to MAX, negative to MIN.
        assert_eq!(Si8::new(1) << 8, Si8::MAX);
        assert_eq!(Si8::new(-1) << 8, Si8::MIN);
        // Shl: headroom > rhs (normal shift).
        assert_eq!(Si8::new(1) << 2, Si8::new(4));
        assert_eq!(Si8::new(-1) << 2, Si8::new(-4));
        // Shl: insufficient headroom -> saturate.
        assert_eq!(Si8::new(64) << 2, Si8::MAX);
        assert_eq!(Si8::new(-64) << 2, Si8::MIN);
        // Shl assign.
        let mut left_shifted = Si8::new(1);
        left_shifted <<= 2;
        assert_eq!(left_shifted, Si8::new(4));

        // Shr: rhs < BITS (normal arithmetic shift).
        assert_eq!(Si8::new(8) >> 2, Si8::new(2));
        assert_eq!(Si8::new(-8) >> 2, Si8::new(-2));
        // Shr: rhs >= BITS sign-extends to 0 or -1.
        assert_eq!(Si8::new(8) >> 8, Si8::ZERO);
        assert_eq!(Si8::new(-1) >> 8, Si8::new(-1));
        let mut right_shifted = Si8::new(8);
        right_shifted >>= 2;
        assert_eq!(right_shifted, Si8::new(2));

        // Neg: normal and saturating.
        assert_eq!(-Si8::new(5), Si8::new(-5));
        assert_eq!(-Si8::new(-5), Si8::new(5));
        assert_eq!(-Si8::MIN, Si8::MAX);
    }

    #[test]
    fn unsigned_shifts() {
        // Shl: normal.
        assert_eq!(Su8::new(1) << 3, Su8::new(8));
        // Shl: rhs >= BITS saturates.
        assert_eq!(Su8::new(1) << 8, Su8::MAX);
        // Shl: leading_zeros < rhs saturates.
        assert_eq!(Su8::new(0b1100_0000) << 2, Su8::MAX);
        let mut left_shifted = Su8::new(1);
        left_shifted <<= 3;
        assert_eq!(left_shifted, Su8::new(8));

        // Shr: normal.
        assert_eq!(Su8::new(8) >> 2, Su8::new(2));
        // Shr: rhs >= BITS yields 0.
        assert_eq!(Su8::new(255) >> 8, Su8::ZERO);
        let mut right_shifted = Su8::new(8);
        right_shifted >>= 2;
        assert_eq!(right_shifted, Su8::new(2));
    }

    #[test]
    fn larger_widths_smoke() {
        // Exercise the macro-generated impls for larger widths to ensure
        // expansions over the full type list are covered.
        assert_eq!(Si16::new(100) + Si16::new(200), Si16::new(300));
        assert_eq!(Su16::new(100) + Su16::new(200), Su16::new(300));
        assert_eq!(Si128::MAX + Si128::new(1), Si128::MAX);
        assert_eq!(Su128::MAX + Su128::new(1), Su128::MAX);
    }
}
