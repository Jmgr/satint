#![no_std]
#![forbid(unsafe_code)]
//! Saturating scalar wrappers for signed and unsigned primitive integers.
//!
//! The concrete aliases, such as [`Si32`] and [`Su32`], wrap Rust's primitive
//! integer types and use saturating arithmetic for the implemented arithmetic
//! operators. Conversions that may lose information are available through
//! [`SaturatingFrom`] and [`TryFrom`].

/// Saturating conversion traits and cross-width conversion impls.
pub mod convert;
/// Signed saturating scalar types.
pub mod si;
/// Unsigned saturating scalar types.
pub mod su;

pub use convert::{SaturatingFrom, SaturatingInto};
pub use si::{Si, Si8, Si16, Si32, Si64, Si128, si8, si16, si32, si64, si128};
pub use su::{Su, Su8, Su16, Su32, Su64, Su128, su8, su16, su32, su64, su128};

macro_rules! define_wrapper {
    ($wrapper:ident) => {
        /// A saturating scalar wrapper around a primitive integer type.
        ///
        /// Concrete aliases such as `Si32` and `Su32` are usually preferred
        /// over naming this generic wrapper directly.
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
        #[repr(transparent)]
        pub struct $wrapper<T>(core::num::Saturating<T>);

        impl<T: core::fmt::Debug> core::fmt::Debug for $wrapper<T> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($wrapper)).field(&self.0.0).finish()
            }
        }

        impl<T> $wrapper<T> {
            /// Creates a scalar from an inner primitive value.
            #[inline]
            pub const fn new(value: T) -> Self {
                Self(core::num::Saturating(value))
            }

            /// Returns the wrapped primitive value.
            #[inline]
            pub const fn into_inner(self) -> T
            where
                T: Copy,
            {
                self.0.0
            }
        }

        #[cfg(feature = "serde")]
        impl<T> serde::Serialize for $wrapper<T>
        where
            T: serde::Serialize,
        {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serde::Serialize::serialize(&self.0.0, serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, T> serde::Deserialize<'de> for $wrapper<T>
        where
            T: serde::Deserialize<'de>,
        {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <T as serde::Deserialize>::deserialize(deserializer).map(Self::new)
            }
        }

        impl<T: core::fmt::Display> core::fmt::Display for $wrapper<T> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<T> From<T> for $wrapper<T> {
            #[inline]
            fn from(value: T) -> Self {
                Self::new(value)
            }
        }

        impl<T: PartialEq> PartialEq<T> for $wrapper<T> {
            #[inline]
            fn eq(&self, other: &T) -> bool {
                self.0.0 == *other
            }
        }

        impl<T: PartialOrd> PartialOrd<T> for $wrapper<T> {
            #[inline]
            fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
                self.0.0.partial_cmp(other)
            }
        }

        $crate::define_wrapper!(@op $wrapper, Add, AddAssign, add, add_assign);
        $crate::define_wrapper!(@op $wrapper, Sub, SubAssign, sub, sub_assign);
        $crate::define_wrapper!(@op $wrapper, Mul, MulAssign, mul, mul_assign);

        impl<T> core::iter::Sum for $wrapper<T>
        where
            Self: core::ops::Add<Output = Self> + Default,
        {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::default(), |a, b| a + b)
            }
        }

        impl<'a, T: 'a> core::iter::Sum<&'a Self> for $wrapper<T>
        where
            Self: core::ops::Add<Output = Self> + Default + Copy,
        {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().fold(Self::default(), |a, b| a + b)
            }
        }
    };

    (@op $wrapper:ident, $op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T> core::ops::$op for $wrapper<T>
        where
            core::num::Saturating<T>: core::ops::$op<Output = core::num::Saturating<T>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                Self(self.0.$method(rhs.0))
            }
        }

        impl<T> core::ops::$op_assign for $wrapper<T>
        where
            core::num::Saturating<T>: core::ops::$op_assign,
        {
            #[inline]
            fn $method_assign(&mut self, rhs: Self) {
                self.0.$method_assign(rhs.0);
            }
        }

        impl<T> core::ops::$op<T> for $wrapper<T>
        where
            core::num::Saturating<T>: core::ops::$op<Output = core::num::Saturating<T>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                Self(self.0.$method(core::num::Saturating(rhs)))
            }
        }

        impl<T> core::ops::$op_assign<T> for $wrapper<T>
        where
            core::num::Saturating<T>: core::ops::$op_assign,
        {
            #[inline]
            fn $method_assign(&mut self, rhs: T) {
                self.0.$method_assign(core::num::Saturating(rhs));
            }
        }
    };
}

pub(crate) use define_wrapper;

macro_rules! scalars {
    ($($ty:ident, $alias:ident, $ctor:ident, $primitive:ty);+ $(;)?) => {
        $(
            /// A concrete saturating scalar over the matching primitive integer.
            pub type $alias = $ty<$primitive>;

            impl From<$alias> for $primitive {
                #[inline]
                fn from(value: $alias) -> Self {
                    value.into_inner()
                }
            }

            impl PartialEq<$alias> for $primitive {
                #[inline]
                fn eq(&self, other: &$alias) -> bool {
                    *self == other.into_inner()
                }
            }

            impl PartialOrd<$alias> for $primitive {
                #[inline]
                fn partial_cmp(&self, other: &$alias) -> Option<core::cmp::Ordering> {
                    self.partial_cmp(&other.into_inner())
                }
            }

            impl $ty<$primitive> {
                /// The minimum representable value for this scalar type.
                pub const MIN: Self = $ctor(<$primitive>::MIN);
                /// The maximum representable value for this scalar type.
                pub const MAX: Self = $ctor(<$primitive>::MAX);
                /// The additive identity value.
                pub const ZERO: Self = $ctor(0);
                /// The multiplicative identity value.
                pub const ONE: Self = $ctor(1);

                /// Divides two scalar values, returning `None` on division by zero
                /// or primitive signed overflow.
                ///
                /// Signed overflow can occur for `MIN / -1`.
                #[inline]
                #[must_use]
                pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                    match self.into_inner().checked_div(rhs.into_inner()) {
                        Some(v) => Some($ctor(v)),
                        None => None,
                    }
                }

                /// Calculates the remainder of two scalar values, returning `None`
                /// on division by zero or primitive signed overflow.
                ///
                /// Signed overflow can occur for `MIN % -1`.
                #[inline]
                #[must_use]
                pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                    match self.into_inner().checked_rem(rhs.into_inner()) {
                        Some(v) => Some($ctor(v)),
                        None => None,
                    }
                }
            }

            impl core::iter::Product for $alias {
                #[inline]
                fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                    iter.fold(Self::ONE, |a, b| a * b)
                }
            }

            impl<'a> core::iter::Product<&'a Self> for $alias {
                #[inline]
                fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                    iter.copied().fold(Self::ONE, |a, b| a * b)
                }
            }

            /// Creates a concrete saturating scalar from a primitive value.
            #[must_use]
            #[inline]
            pub const fn $ctor(value: $primitive) -> $alias {
                $ty::new(value)
            }
        )+
    };
}

pub(crate) use scalars;
