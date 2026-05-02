//! Signed saturating scalar aliases and constructors.

use core::{num::Saturating, ops::Neg};

use crate::{
    define_wrapper, scalars,
    su::{Su8, Su16, Su32, Su64, Su128},
};

define_wrapper!(Si);

scalars! {
    Si, Si8, si8, i8;
    Si, Si16, si16, i16;
    Si, Si32, si32, i32;
    Si, Si64, si64, i64;
    Si, Si128, si128, i128;
}

impl<T> Neg for Si<T>
where
    Saturating<T>: Neg<Output = Saturating<T>>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

macro_rules! signed_scalar_methods {
    ($($alias:ident, $ctor:ident, $primitive:ty, $unsigned_alias:ty, $unsigned_ctor:path);+ $(;)?) => {
        $(
            impl $alias {
                /// Computes the absolute value, saturating `MIN` to `MAX`.
                #[inline]
                #[must_use]
                pub const fn abs(self) -> Self {
                    $ctor(self.into_inner().saturating_abs())
                }

                /// Computes the absolute value as an unsigned scalar.
                #[inline]
                #[must_use]
                pub const fn unsigned_abs(self) -> $unsigned_alias {
                    $unsigned_ctor(self.into_inner().unsigned_abs())
                }

                /// Computes the absolute difference between `self` and `rhs`.
                #[inline]
                #[must_use]
                pub const fn abs_diff(self, rhs: Self) -> $unsigned_alias {
                    $unsigned_ctor(self.into_inner().abs_diff(rhs.into_inner()))
                }

                /// Computes the absolute value, returning `None` for `MIN`.
                #[inline]
                #[must_use]
                pub const fn checked_abs(self) -> Option<Self> {
                    match self.into_inner().checked_abs() {
                        Some(v) => Some($ctor(v)),
                        None => None,
                    }
                }

                /// Returns a number representing the sign of `self`.
                #[inline]
                #[must_use]
                pub const fn signum(self) -> Self {
                    $ctor(self.into_inner().signum())
                }

                /// Returns `true` if `self` is positive.
                #[inline]
                #[must_use]
                pub const fn is_positive(self) -> bool {
                    self.into_inner().is_positive()
                }

                /// Returns `true` if `self` is negative.
                #[inline]
                #[must_use]
                pub const fn is_negative(self) -> bool {
                    self.into_inner().is_negative()
                }

                /// Returns the integer square root, or `None` if `self` is negative.
                #[inline]
                #[must_use]
                pub const fn checked_isqrt(self) -> Option<Self> {
                    match self.into_inner().checked_isqrt() {
                        Some(v) => Some($ctor(v)),
                        None => None,
                    }
                }
            }
        )+
    };
}

signed_scalar_methods! {
    Si8, si8, i8, Su8, crate::su8;
    Si16, si16, i16, Su16, crate::su16;
    Si32, si32, i32, Su32, crate::su32;
    Si64, si64, i64, Su64, crate::su64;
    Si128, si128, i128, Su128, crate::su128;
}
