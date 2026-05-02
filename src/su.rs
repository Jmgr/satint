//! Unsigned saturating scalar aliases and constructors.

use crate::{define_wrapper, scalars};

define_wrapper!(Su);

scalars! {
    Su, Su8, su8, u8;
    Su, Su16, su16, u16;
    Su, Su32, su32, u32;
    Su, Su64, su64, u64;
    Su, Su128, su128, u128;
}

macro_rules! unsigned_scalar_methods {
    ($($alias:ident, $ctor:ident);+ $(;)?) => {
        $(
            impl $alias {
                /// Computes the absolute difference between `self` and `rhs`.
                #[inline]
                #[must_use]
                pub const fn abs_diff(self, rhs: Self) -> Self {
                    $ctor(self.into_inner().abs_diff(rhs.into_inner()))
                }

                /// Returns `true` if `self` is a power of two.
                #[inline]
                #[must_use]
                pub const fn is_power_of_two(self) -> bool {
                    self.into_inner().is_power_of_two()
                }

                /// Returns the smallest power of two greater than or equal to `self`,
                /// saturating at `MAX` if the primitive operation would overflow.
                #[inline]
                #[must_use]
                pub const fn next_power_of_two(self) -> Self {
                    match self.checked_next_power_of_two() {
                        Some(v) => v,
                        None => Self::MAX,
                    }
                }

                /// Returns the smallest power of two greater than or equal to `self`,
                /// or `None` if the primitive operation would overflow.
                #[inline]
                #[must_use]
                pub const fn checked_next_power_of_two(self) -> Option<Self> {
                    match self.into_inner().checked_next_power_of_two() {
                        Some(v) => Some($ctor(v)),
                        None => None,
                    }
                }

                /// Returns the integer square root.
                #[inline]
                #[must_use]
                pub const fn isqrt(self) -> Self {
                    $ctor(self.into_inner().isqrt())
                }
            }
        )+
    };
}

unsigned_scalar_methods! {
    Su8, su8;
    Su16, su16;
    Su32, su32;
    Su64, su64;
    Su128, su128;
}
