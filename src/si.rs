use core::num::Saturating;

use crate::{
    common::{
        Inner, SaturatingFrom, generate_from_primitive_to_wrapper,
        generate_from_wrapper_to_primitive, generate_from_wrapper_to_wrapper,
        generate_saturating_from_float_to_wrapper,
        generate_saturating_from_signed_primitive_to_signed_wrapper,
        generate_saturating_from_signed_wrapper_to_unsigned_primitive,
        generate_saturating_from_unsigned_primitive_to_signed_wrapper,
        generate_saturating_from_wrapper_to_primitive, generate_saturating_from_wrapper_to_wrapper,
        generate_saturating_wrapper,
    },
    su::{Su8, Su16, Su32, Su64, Su128},
};

generate_saturating_wrapper!(Si8; si8; i8);
generate_saturating_wrapper!(Si16; si16; i16);
generate_saturating_wrapper!(Si32; si32; i32);
generate_saturating_wrapper!(Si64; si64; i64);
generate_saturating_wrapper!(Si128; si128; i128);

macro_rules! generate_signed_functions {
    ($($name:ident; $unsigned_name:ident)+) => {
        $(
            impl $name {
                #[doc = concat!(
                    "Computes the absolute value, saturating at [`", stringify!($name), "::MAX`].\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(-5).abs(), ", stringify!($name), "::new(5));\n",
                    "assert_eq!(", stringify!($name), "::MIN.abs(), ", stringify!($name), "::MAX);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn abs(self) -> Self {
                    Self::new(self.0.0.saturating_abs())
                }

                #[doc = concat!(
                    "Computes the absolute value as an unsigned scalar.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::{", stringify!($name), ", ", stringify!($unsigned_name), "};\n\n",
                    "assert_eq!(", stringify!($name), "::new(-5).unsigned_abs(), ", stringify!($unsigned_name), "::new(5));\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn unsigned_abs(self) -> $unsigned_name {
                    $unsigned_name::new(self.into_inner().unsigned_abs())
                }

                #[doc = concat!(
                    "Computes the absolute difference between `self` and `rhs`.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::{", stringify!($name), ", ", stringify!($unsigned_name), "};\n\n",
                    "assert_eq!(", stringify!($name), "::new(-10).abs_diff(", stringify!($name), "::new(5)), ", stringify!($unsigned_name), "::new(15));\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn abs_diff(self, rhs: Self) -> $unsigned_name {
                    $unsigned_name::new(self.into_inner().abs_diff(rhs.into_inner()))
                }

                #[doc = concat!(
                    "Computes the absolute value, returning `None` for [`", stringify!($name), "::MIN`].\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(-5).checked_abs(), Some(", stringify!($name), "::new(5)));\n",
                    "assert_eq!(", stringify!($name), "::MIN.checked_abs(), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_abs(self) -> Option<Self> {
                    match self.into_inner().checked_abs() {
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                #[doc = concat!(
                    "Returns a number representing the sign of `self`.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(-5).signum(), ", stringify!($name), "::new(-1));\n",
                    "assert_eq!(", stringify!($name), "::ZERO.signum(), ", stringify!($name), "::ZERO);\n",
                    "assert_eq!(", stringify!($name), "::new(5).signum(), ", stringify!($name), "::ONE);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn signum(self) -> Self {
                    Self::new(self.into_inner().signum())
                }

                #[doc = concat!(
                    "Returns `true` if `self` is positive.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert!(", stringify!($name), "::new(5).is_positive());\n",
                    "assert!(!", stringify!($name), "::new(-5).is_positive());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn is_positive(self) -> bool {
                    self.into_inner().is_positive()
                }

                #[doc = concat!(
                    "Returns `true` if `self` is negative.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert!(", stringify!($name), "::new(-5).is_negative());\n",
                    "assert!(!", stringify!($name), "::new(5).is_negative());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn is_negative(self) -> bool {
                    self.into_inner().is_negative()
                }

                #[doc = concat!(
                    "Returns the integer square root, or `None` if `self` is negative.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(16).checked_isqrt(), Some(", stringify!($name), "::new(4)));\n",
                    "assert_eq!(", stringify!($name), "::new(-1).checked_isqrt(), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_isqrt(self) -> Option<Self> {
                    match self.into_inner().checked_isqrt() {
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                #[doc = concat!(
                    "Converts to the same-width unsigned wrapper, saturating negative values to zero.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::{", stringify!($name), ", ", stringify!($unsigned_name), "};\n\n",
                    "assert_eq!(", stringify!($name), "::new(42).to_unsigned(), ", stringify!($unsigned_name), "::new(42));\n",
                    "assert_eq!(", stringify!($name), "::new(-1).to_unsigned(), ", stringify!($unsigned_name), "::ZERO);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn to_unsigned(self) -> $unsigned_name {
                    if self.is_negative() {
                        return $unsigned_name::ZERO;
                    }

                    self.unsigned_abs()
                }
            }

            generate_saturating_from_signed_primitive_to_signed_wrapper!($name; i8, i16, i32, i64, i128, isize);
            generate_saturating_from_unsigned_primitive_to_signed_wrapper!($name; u8, u16, u32, u64, u128, usize);
            generate_saturating_from_wrapper_to_primitive!($name; i128; i8, i16, i32, i64, isize);
            generate_saturating_from_signed_wrapper_to_unsigned_primitive!($name; u8, u16, u32, u64, u128, usize);
            generate_saturating_from_float_to_wrapper!($name; f32, f64);
        )+
    };
}

generate_signed_functions!(Si8; Su8);
generate_signed_functions!(Si16; Su16);
generate_signed_functions!(Si32; Su32);
generate_signed_functions!(Si64; Su64);
generate_signed_functions!(Si128; Su128);

generate_from_primitive_to_wrapper!(Si16; i8, u8);
generate_from_primitive_to_wrapper!(Si32; i8, i16, u8, u16);
generate_from_primitive_to_wrapper!(Si64; i8, i16, i32, u8, u16, u32);
generate_from_primitive_to_wrapper!(Si128; i8, i16, i32, i64, u8, u16, u32, u64);

generate_from_wrapper_to_primitive!(Si8; i16, i32, i64, i128, isize, f32, f64);
generate_from_wrapper_to_primitive!(Si16; i32, i64, i128, isize, f32, f64);
generate_from_wrapper_to_primitive!(Si32; i64, i128, f64);
generate_from_wrapper_to_primitive!(Si64; i128);

generate_from_wrapper_to_wrapper!(Si16; Si8);
generate_from_wrapper_to_wrapper!(Si32; Si8, Si16);
generate_from_wrapper_to_wrapper!(Si64; Si8, Si16, Si32);
generate_from_wrapper_to_wrapper!(Si128; Si8, Si16, Si32, Si64);

generate_saturating_from_wrapper_to_wrapper!(Si8; i128; Si16, Si32, Si64, Si128);
generate_saturating_from_wrapper_to_wrapper!(Si16; i128; Si8,  Si32, Si64, Si128);
generate_saturating_from_wrapper_to_wrapper!(Si32; i128; Si8,  Si16, Si64, Si128);
generate_saturating_from_wrapper_to_wrapper!(Si64; i128; Si8,  Si16, Si32, Si128);
generate_saturating_from_wrapper_to_wrapper!(Si128; i128; Si8,  Si16, Si32, Si64);

#[cfg(test)]
mod tests {
    use crate::{
        common::{Inner, SaturatingFrom},
        si::{Si8, Si16, Si32, Si64, Si128},
        su::{Su8, Su16, Su32, Su64, Su128},
    };

    macro_rules! test_signed_suite {
        ($($mod_name:ident; $name:ident; $unsigned:ident)+) => {
            $(
                mod $mod_name {
                    use super::*;
                    type T = <$name as Inner>::Inner;

                    #[test]
                    fn test_basics() {
                        assert_eq!($name::ZERO.into_inner(), 0);
                        assert_eq!($name::ONE.into_inner(), 1);
                        assert_eq!($name::MIN.into_inner(), T::MIN);
                        assert_eq!($name::MAX.into_inner(), T::MAX);
                    }

                    #[test]
                    fn test_methods() {
                        // abs (saturates at MIN since |MIN| > MAX).
                        assert_eq!($name::new(-5).abs(), $name::new(5));
                        assert_eq!($name::new(5).abs(), $name::new(5));
                        assert_eq!($name::ZERO.abs(), $name::ZERO);
                        assert_eq!($name::MIN.abs(), $name::MAX);

                        // checked_abs.
                        assert_eq!($name::new(-5).checked_abs(), Some($name::new(5)));
                        assert_eq!($name::new(5).checked_abs(), Some($name::new(5)));
                        assert_eq!($name::MIN.checked_abs(), None);

                        // unsigned_abs.
                        assert_eq!($name::new(-5).unsigned_abs(), $unsigned::new(5));
                        assert_eq!($name::new(5).unsigned_abs(), $unsigned::new(5));

                        // abs_diff (returns unsigned wrapper).
                        let a = $name::new(-10);
                        let b = $name::new(15);
                        assert_eq!(a.abs_diff(b), $unsigned::new(25));
                        assert_eq!(b.abs_diff(a), $unsigned::new(25));

                        // signum.
                        assert_eq!($name::new(-5).signum(), $name::new(-1));
                        assert_eq!($name::ZERO.signum(), $name::ZERO);
                        assert_eq!($name::new(5).signum(), $name::ONE);

                        // is_positive / is_negative.
                        assert!($name::new(5).is_positive());
                        assert!(!$name::new(-5).is_positive());
                        assert!(!$name::ZERO.is_positive());
                        assert!($name::new(-5).is_negative());
                        assert!(!$name::new(5).is_negative());
                        assert!(!$name::ZERO.is_negative());

                        // checked_isqrt.
                        assert_eq!($name::new(16).checked_isqrt(), Some($name::new(4)));
                        assert_eq!($name::new(15).checked_isqrt(), Some($name::new(3)));
                        assert_eq!($name::ZERO.checked_isqrt(), Some($name::ZERO));
                        assert_eq!($name::new(-1).checked_isqrt(), None);
                    }

                    #[test]
                    fn test_to_unsigned() {
                        type U = <$unsigned as Inner>::Inner;
                        // Non-negative passes through.
                        assert_eq!($name::new(10).to_unsigned(), $unsigned::new(10));
                        assert_eq!($name::ZERO.to_unsigned(), $unsigned::ZERO);
                        // MAX maps to its unsigned representation.
                        assert_eq!($name::MAX.to_unsigned(), $unsigned::new(T::MAX as U));
                        // Negative saturates to 0.
                        assert_eq!($name::new(-1).to_unsigned(), $unsigned::ZERO);
                        assert_eq!($name::MIN.to_unsigned(), $unsigned::ZERO);
                    }

                    #[test]
                    fn test_saturating_conversions() {
                        // Large unsigned source saturates at signed MAX.
                        assert_eq!($name::saturating_from(u128::MAX), $name::MAX);
                        // Below dest MIN saturates at MIN (via wrapper-to-wrapper).
                        assert_eq!($name::saturating_from(Si128::MIN), $name::MIN);
                        // In-range value passes through.
                        assert_eq!($name::saturating_from(42_u128), $name::new(42));
                    }

                    #[test]
                    fn test_neg() {
                        assert_eq!(-$name::new(5), $name::new(-5));
                        assert_eq!(-$name::new(-5), $name::new(5));
                        assert_eq!(-$name::ZERO, $name::ZERO);
                        // Neg of MIN saturates at MAX since |MIN| > MAX.
                        assert_eq!(-$name::MIN, $name::MAX);
                    }
                }
            )+
        };
    }

    test_signed_suite!(
        si8_tests;   Si8;   Su8
        si16_tests;  Si16;  Su16
        si32_tests;  Si32;  Su32
        si64_tests;  Si64;  Su64
        si128_tests; Si128; Su128
    );
}
