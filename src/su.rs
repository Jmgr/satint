use core::num::Saturating;

use crate::{
    common::{
        Inner, SaturatingFrom, generate_from_primitive_to_wrapper,
        generate_from_wrapper_to_primitive, generate_from_wrapper_to_wrapper,
        generate_saturating_from_float_to_wrapper,
        generate_saturating_from_signed_primitive_to_unsigned_wrapper,
        generate_saturating_from_unsigned_primitive_to_unsigned_wrapper,
        generate_saturating_from_unsigned_wrapper_to_signed_primitive,
        generate_saturating_from_wrapper_to_primitive, generate_saturating_from_wrapper_to_wrapper,
        generate_saturating_wrapper,
    },
    si::{Si8, Si16, Si32, Si64, Si128},
};

generate_saturating_wrapper!(Su8; su8; u8);
generate_saturating_wrapper!(Su16; su16; u16);
generate_saturating_wrapper!(Su32; su32; u32);
generate_saturating_wrapper!(Su64; su64; u64);
generate_saturating_wrapper!(Su128; su128; u128);

macro_rules! generate_unsigned_functions {
    ($($name:ident; $inner:ty; $signed_name:ident; $signed_inner:ty)+) => {
        $(
            impl $name {
                /// Computes the absolute difference between `self` and `rhs`.
                #[inline]
                #[must_use]
                pub const fn abs_diff(self, rhs: Self) -> Self {
                    Self::new(self.into_inner().abs_diff(rhs.into_inner()))
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
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                /// Returns the integer square root.
                #[inline]
                #[must_use]
                pub const fn isqrt(self) -> Self {
                    Self::new(self.into_inner().isqrt())
                }

                #[inline]
                #[must_use]
                pub const fn to_signed(self) -> $signed_name {
                    let value = self.0.0;
                    let cap = <$signed_inner>::MAX as $inner;
                    if value > cap {
                        $signed_name::MAX
                    } else {
                        $signed_name::new(value as $signed_inner)
                    }
                }
            }

            generate_saturating_from_unsigned_primitive_to_unsigned_wrapper!($name; u8, u16, u32, u64, u128, usize);
            generate_saturating_from_signed_primitive_to_unsigned_wrapper!($name; i8, i16, i32, i64, i128, isize);
            generate_saturating_from_wrapper_to_primitive!($name; u128; u8, u16, u32, u64, usize);
            generate_saturating_from_unsigned_wrapper_to_signed_primitive!($name; i8, i16, i32, i64, i128, isize);
            generate_saturating_from_float_to_wrapper!($name; f32, f64);
        )+
    };
}

generate_unsigned_functions!(Su8; u8; Si8; i8);
generate_unsigned_functions!(Su16; u16; Si16; i16);
generate_unsigned_functions!(Su32; u32; Si32; i32);
generate_unsigned_functions!(Su64; u64; Si64; i64);
generate_unsigned_functions!(Su128; u128; Si128; i128);

generate_from_primitive_to_wrapper!(Su16; u8);
generate_from_primitive_to_wrapper!(Su32; u8, u16);
generate_from_primitive_to_wrapper!(Su64; u8, u16, u32);
generate_from_primitive_to_wrapper!(Su128; u8, u16, u32, u64);

generate_from_wrapper_to_primitive!(Su8; u16, u32, u64, u128, usize, isize, f32, f64);
generate_from_wrapper_to_primitive!(Su16; u32, u64, u128, usize, f32, f64);
generate_from_wrapper_to_primitive!(Su32; u64, u128, f64);
generate_from_wrapper_to_primitive!(Su64; u128);

generate_from_wrapper_to_wrapper!(Su16; Su8);
generate_from_wrapper_to_wrapper!(Su32; Su8, Su16);
generate_from_wrapper_to_wrapper!(Su64; Su8, Su16, Su32);
generate_from_wrapper_to_wrapper!(Su128; Su8, Su16, Su32, Su64);

generate_saturating_from_wrapper_to_wrapper!(Su8;   u128; Su16, Su32, Su64, Su128);
generate_saturating_from_wrapper_to_wrapper!(Su16;  u128; Su8,  Su32, Su64, Su128);
generate_saturating_from_wrapper_to_wrapper!(Su32;  u128; Su8,  Su16, Su64, Su128);
generate_saturating_from_wrapper_to_wrapper!(Su64;  u128; Su8,  Su16, Su32, Su128);
generate_saturating_from_wrapper_to_wrapper!(Su128; u128; Su8,  Su16, Su32, Su64);

#[cfg(test)]
mod tests {
    use crate::{
        common::{Inner, SaturatingFrom},
        si::{Si8, Si16, Si32, Si64, Si128},
        su::{Su8, Su16, Su32, Su64, Su128},
    };

    macro_rules! test_unsigned_suite {
        ($($mod_name:ident; $name:ident; $signed:ident)+) => {
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
                        let a = $name::new(10);
                        let b = $name::new(25);
                        assert_eq!(a.abs_diff(b), $name::new(15));
                        assert_eq!(b.abs_diff(a), $name::new(15));

                        assert!($name::new(16).is_power_of_two());
                        assert!(!$name::new(10).is_power_of_two());

                        assert_eq!($name::new(15).next_power_of_two(), $name::new(16));
                        assert_eq!($name::new(16).next_power_of_two(), $name::new(16));
                        assert_eq!($name::MAX.next_power_of_two(), $name::MAX);

                        assert_eq!($name::new(16).isqrt(), $name::new(4));
                        assert_eq!($name::new(15).isqrt(), $name::new(3));
                    }

                    #[test]
                    fn test_to_signed() {
                        type S = <$signed as Inner>::Inner;
                        assert_eq!($name::new(10).to_signed(), $signed::new(10));
                        assert_eq!($name::MAX.to_signed(), $signed::MAX);

                        let cap = S::MAX as T;
                        assert_eq!($name::new(cap).to_signed(), $signed::new(S::MAX));
                        if cap < T::MAX {
                            assert_eq!($name::new(cap + 1).to_signed(), $signed::MAX);
                        }
                    }

                    #[test]
                    fn test_saturating_conversions() {
                        // Signed to Unsigned saturation (negatives to 0)
                        assert_eq!($name::saturating_from(-1i8), $name::ZERO);
                        assert_eq!($name::saturating_from(-128i8), $name::ZERO);
                    }
                }
            )+
        };
    }

    test_unsigned_suite!(
        su8_tests;   Su8;   Si8
        su16_tests;  Su16;  Si16
        su32_tests;  Su32;  Si32
        su64_tests;  Su64;  Si64
        su128_tests; Su128; Si128
    );
}
