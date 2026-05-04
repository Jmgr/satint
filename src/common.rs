/// Lossy conversion that clamps the value to the destination range.
///
/// Out-of-range positive values saturate to the destination's `MAX`, and
/// out-of-range negative values saturate to its `MIN` (or `0` for unsigned
/// targets). Use `From`/`Into` for lossless conversions.
///
/// # Examples
///
/// ```
/// use satint::{SaturatingFrom, Si8, Su8};
///
/// assert_eq!(Si8::saturating_from(200_i16), Si8::MAX);
/// assert_eq!(Su8::saturating_from(-1_i16), Su8::ZERO);
/// ```
pub trait SaturatingFrom<T>: Sized {
    /// Converts `value` into `Self`, clamping to the destination range when
    /// the source value cannot be represented exactly.
    ///
    /// # Examples
    ///
    /// ```
    /// use satint::{SaturatingFrom, Su8};
    ///
    /// assert_eq!(Su8::saturating_from(300_u16), Su8::MAX);
    /// ```
    fn saturating_from(value: T) -> Self;
}

/// Reciprocal of [`SaturatingFrom`]. Provided automatically via a blanket impl.
///
/// # Examples
///
/// ```
/// use satint::{SaturatingInto, Si8};
///
/// let value: Si8 = 200_i16.saturating_into();
/// assert_eq!(value, Si8::MAX);
/// ```
pub trait SaturatingInto<U>: Sized {
    /// Converts `self` into `U`, clamping to the destination range when the
    /// source value cannot be represented exactly.
    ///
    /// # Examples
    ///
    /// ```
    /// use satint::{SaturatingInto, Su8};
    ///
    /// let value: Su8 = (-1_i16).saturating_into();
    /// assert_eq!(value, Su8::ZERO);
    /// ```
    fn saturating_into(self) -> U;
}

impl<T, U> SaturatingInto<U> for T
where
    U: SaturatingFrom<T>,
{
    #[inline]
    fn saturating_into(self) -> U {
        U::saturating_from(self)
    }
}

impl<T> SaturatingFrom<T> for T {
    #[inline]
    fn saturating_from(value: T) -> Self {
        value
    }
}

pub(crate) trait Inner {
    type Inner;
}

macro_rules! generate_saturating_wrapper {
    ($($name:ident; $const:ident; $inner:ty)+) => {
        $(
            #[doc = concat!(
                "A saturating wrapper around [`",
                stringify!($inner),
                "`].\n\n",
                "# Examples\n\n",
                "```rust\n",
                "use satint::{", stringify!($name), ", ", stringify!($const), "};\n\n",
                "let value = ", stringify!($const), "(42);\n",
                "assert_eq!(value, ", stringify!($name), "::new(42));\n",
                "assert_eq!(value.into_inner(), 42);\n",
                "```"
            )]
            #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
            #[repr(transparent)]
            pub struct $name(Saturating<$inner>);

            impl Inner for $name {
                type Inner = $inner;
            }

            impl From<$inner> for $name {
                #[inline]
                fn from(value: $inner) -> Self {
                    Self::new(value)
                }
            }

            impl From<$name> for $inner {
                #[inline]
                fn from(value: $name) -> Self {
                    value.0.0
                }
            }

            impl PartialEq<$inner> for $name {
                #[inline]
                fn eq(&self, other: &$inner) -> bool {
                    self.into_inner() == *other
                }
            }

            impl PartialOrd<$inner> for $name {
                #[inline]
                fn partial_cmp(&self, other: &$inner) -> Option<core::cmp::Ordering> {
                    self.into_inner().partial_cmp(other)
                }
            }

            impl PartialEq<$name> for $inner {
                #[inline]
                fn eq(&self, other: &$name) -> bool {
                    self == &other.into_inner()
                }
            }

            impl PartialOrd<$name> for $inner {
                #[inline]
                fn partial_cmp(&self, other: &$name) -> Option<core::cmp::Ordering> {
                    self.partial_cmp(&other.into_inner())
                }
            }

            impl core::str::FromStr for $name {
                type Err = <$inner as core::str::FromStr>::Err;
                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    <$inner>::from_str(s).map(Self::new)
                }
            }

            impl core::fmt::Debug for $name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_tuple(stringify!($name)).field(&self.0.0).finish()
                }
            }

            impl core::fmt::Display for $name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl core::iter::Sum for $name {
                #[inline]
                fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                    iter.fold(Self::default(), |a, b| a + b)
                }
            }

            impl<'a> core::iter::Sum<&'a Self> for $name {
                #[inline]
                fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                    iter.copied().fold(Self::default(), |a, b| a + b)
                }
            }

            impl core::iter::Product for $name {
                #[inline]
                fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                    iter.fold(Self::ONE, |a, b| a * b)
                }
            }

            impl<'a> core::iter::Product<&'a Self> for $name {
                #[inline]
                fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                    iter.copied().fold(Self::ONE, |a, b| a * b)
                }
            }

            impl core::ops::BitAnd for $name {
                type Output = Self;

                #[inline]
                fn bitand(self, rhs: Self) -> Self::Output {
                    Self::new(self.into_inner() & rhs.into_inner())
                }
            }

            impl core::ops::BitAndAssign for $name {
                #[inline]
                fn bitand_assign(&mut self, rhs: Self) {
                    *self = *self & rhs;
                }
            }

            impl core::ops::BitAnd<$inner> for $name {
                type Output = Self;

                #[inline]
                fn bitand(self, rhs: $inner) -> Self::Output {
                    Self::new(self.into_inner() & rhs)
                }
            }

            impl core::ops::BitAndAssign<$inner> for $name {
                #[inline]
                fn bitand_assign(&mut self, rhs: $inner) {
                    *self = *self & rhs;
                }
            }

            impl core::ops::BitOr for $name {
                type Output = Self;

                #[inline]
                fn bitor(self, rhs: Self) -> Self::Output {
                    Self::new(self.into_inner() | rhs.into_inner())
                }
            }

            impl core::ops::BitOrAssign for $name {
                #[inline]
                fn bitor_assign(&mut self, rhs: Self) {
                    *self = *self | rhs;
                }
            }

            impl core::ops::BitOr<$inner> for $name {
                type Output = Self;

                #[inline]
                fn bitor(self, rhs: $inner) -> Self::Output {
                    Self::new(self.into_inner() | rhs)
                }
            }

            impl core::ops::BitOrAssign<$inner> for $name {
                #[inline]
                fn bitor_assign(&mut self, rhs: $inner) {
                    *self = *self | rhs;
                }
            }

            impl core::ops::BitXor for $name {
                type Output = Self;

                #[inline]
                fn bitxor(self, rhs: Self) -> Self::Output {
                    Self::new(self.into_inner() ^ rhs.into_inner())
                }
            }

            impl core::ops::BitXorAssign for $name {
                #[inline]
                fn bitxor_assign(&mut self, rhs: Self) {
                    *self = *self ^ rhs;
                }
            }

            impl core::ops::BitXor<$inner> for $name {
                type Output = Self;

                #[inline]
                fn bitxor(self, rhs: $inner) -> Self::Output {
                    Self::new(self.into_inner() ^ rhs)
                }
            }

            impl core::ops::BitXorAssign<$inner> for $name {
                #[inline]
                fn bitxor_assign(&mut self, rhs: $inner) {
                    *self = *self ^ rhs;
                }
            }

            impl core::ops::Not for $name {
                type Output = Self;

                #[inline]
                fn not(self) -> Self::Output {
                    Self::new(!self.into_inner())
                }
            }

            impl $name {
                /// The size of this scalar type in bits.
                pub const BITS: u32 = <$inner>::BITS;
                /// The minimum representable value for this scalar type.
                pub const MIN: Self = $name::new(<$inner>::MIN);
                /// The maximum representable value for this scalar type.
                pub const MAX: Self = $name::new(<$inner>::MAX);
                /// The additive identity value.
                pub const ZERO: Self = $name::new(0);
                /// The multiplicative identity value.
                pub const ONE: Self = $name::new(1);

                #[doc = concat!(
                    "Creates a new [`", stringify!($name), "`] from an inner [`",
                    stringify!($inner), "`] value.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(value.into_inner(), 42);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn new(value: $inner) -> Self {
                    Self(Saturating(value))
                }

                #[doc = concat!(
                    "Returns the wrapped [`", stringify!($inner), "`] value.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(42).into_inner(), 42);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn into_inner(self) -> $inner {
                    self.0.0
                }

                #[cfg(feature = "serde")]
                #[inline]
                #[must_use]
                pub(crate) const fn as_inner(&self) -> &$inner {
                    &self.0.0
                }

                #[doc = concat!(
                    "Returns the number of ones in the binary representation.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b1010);\n",
                    "assert_eq!(value.count_ones(), value.into_inner().count_ones());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn count_ones(self) -> u32 {
                    self.into_inner().count_ones()
                }

                #[doc = concat!(
                    "Returns the number of zeros in the binary representation.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b1010);\n",
                    "assert_eq!(value.count_zeros(), value.into_inner().count_zeros());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn count_zeros(self) -> u32 {
                    self.into_inner().count_zeros()
                }

                #[doc = concat!(
                    "Returns the number of leading zeros in the binary representation.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b1010);\n",
                    "assert_eq!(value.leading_zeros(), value.into_inner().leading_zeros());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn leading_zeros(self) -> u32 {
                    self.into_inner().leading_zeros()
                }

                #[doc = concat!(
                    "Returns the number of leading ones in the binary representation.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::MAX;\n",
                    "assert_eq!(value.leading_ones(), value.into_inner().leading_ones());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn leading_ones(self) -> u32 {
                    self.into_inner().leading_ones()
                }

                #[doc = concat!(
                    "Returns the number of trailing zeros in the binary representation.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b1000);\n",
                    "assert_eq!(value.trailing_zeros(), value.into_inner().trailing_zeros());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn trailing_zeros(self) -> u32 {
                    self.into_inner().trailing_zeros()
                }

                #[doc = concat!(
                    "Returns the number of trailing ones in the binary representation.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b1011);\n",
                    "assert_eq!(value.trailing_ones(), value.into_inner().trailing_ones());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn trailing_ones(self) -> u32 {
                    self.into_inner().trailing_ones()
                }

                #[doc = concat!(
                    "Reverses the order of bits.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b0001);\n",
                    "assert_eq!(value.reverse_bits().into_inner(), value.into_inner().reverse_bits());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn reverse_bits(self) -> Self {
                    Self::new(self.into_inner().reverse_bits())
                }

                #[doc = concat!(
                    "Shifts bits to the left by `n`, wrapping the truncated bits to ",
                    "the end of the result.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b0001);\n",
                    "assert_eq!(value.rotate_left(1).into_inner(), value.into_inner().rotate_left(1));\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn rotate_left(self, n: u32) -> Self {
                    Self::new(self.into_inner().rotate_left(n))
                }

                #[doc = concat!(
                    "Shifts bits to the right by `n`, wrapping the truncated bits to ",
                    "the beginning of the result.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(0b0010);\n",
                    "assert_eq!(value.rotate_right(1).into_inner(), value.into_inner().rotate_right(1));\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn rotate_right(self, n: u32) -> Self {
                    Self::new(self.into_inner().rotate_right(n))
                }

                #[doc = concat!(
                    "Reverses the byte order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(value.swap_bytes().into_inner(), value.into_inner().swap_bytes());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn swap_bytes(self) -> Self {
                    Self::new(self.into_inner().swap_bytes())
                }

                #[doc = concat!(
                    "Converts from big-endian to the target's native endian.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let native = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(", stringify!($name), "::from_be(native.to_be()), native);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn from_be(value: Self) -> Self {
                    Self::new(<$inner>::from_be(value.into_inner()))
                }

                #[doc = concat!(
                    "Converts from little-endian to the target's native endian.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let native = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(", stringify!($name), "::from_le(native.to_le()), native);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn from_le(value: Self) -> Self {
                    Self::new(<$inner>::from_le(value.into_inner()))
                }

                #[doc = concat!(
                    "Converts `self` to big-endian from the target's native endian.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let native = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(native.to_be().into_inner(), native.into_inner().to_be());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn to_be(self) -> Self {
                    Self::new(self.into_inner().to_be())
                }

                #[doc = concat!(
                    "Converts `self` to little-endian from the target's native endian.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let native = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(native.to_le().into_inner(), native.into_inner().to_le());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn to_le(self) -> Self {
                    Self::new(self.into_inner().to_le())
                }

                #[doc = concat!(
                    "Returns the memory representation as a byte array in big-endian order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(value.to_be_bytes(), value.into_inner().to_be_bytes());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn to_be_bytes(self) -> [u8; core::mem::size_of::<$inner>()] {
                    self.into_inner().to_be_bytes()
                }

                #[doc = concat!(
                    "Returns the memory representation as a byte array in little-endian order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(value.to_le_bytes(), value.into_inner().to_le_bytes());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn to_le_bytes(self) -> [u8; core::mem::size_of::<$inner>()] {
                    self.into_inner().to_le_bytes()
                }

                #[doc = concat!(
                    "Returns the memory representation as a byte array in native-endian order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(value.to_ne_bytes(), value.into_inner().to_ne_bytes());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn to_ne_bytes(self) -> [u8; core::mem::size_of::<$inner>()] {
                    self.into_inner().to_ne_bytes()
                }

                #[doc = concat!(
                    "Creates a scalar from a byte array in big-endian order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(", stringify!($name), "::from_be_bytes(value.to_be_bytes()), value);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn from_be_bytes(
                    bytes: [u8; core::mem::size_of::<$inner>()],
                ) -> Self {
                    Self::new(<$inner>::from_be_bytes(bytes))
                }

                #[doc = concat!(
                    "Creates a scalar from a byte array in little-endian order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(", stringify!($name), "::from_le_bytes(value.to_le_bytes()), value);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn from_le_bytes(
                    bytes: [u8; core::mem::size_of::<$inner>()],
                ) -> Self {
                    Self::new(<$inner>::from_le_bytes(bytes))
                }

                #[doc = concat!(
                    "Creates a scalar from a byte array in native-endian order.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "let value = ", stringify!($name), "::new(42);\n",
                    "assert_eq!(", stringify!($name), "::from_ne_bytes(value.to_ne_bytes()), value);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn from_ne_bytes(
                    bytes: [u8; core::mem::size_of::<$inner>()],
                ) -> Self {
                    Self::new(<$inner>::from_ne_bytes(bytes))
                }

                #[doc = concat!(
                    "Divides two scalar values, returning `None` on division by zero ",
                    "or primitive signed overflow.\n\n",
                    "Signed overflow can occur for `MIN / -1`.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_div(", stringify!($name), "::new(2)), Some(", stringify!($name), "::new(5)));\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_div(", stringify!($name), "::ZERO), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                    match self.into_inner().checked_div(rhs.into_inner()) {
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                #[doc = concat!(
                    "Calculates Euclidean division, returning `None` on division by ",
                    "zero or primitive signed overflow.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_div_euclid(", stringify!($name), "::new(3)), Some(", stringify!($name), "::new(3)));\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_div_euclid(", stringify!($name), "::ZERO), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                    match self.into_inner().checked_div_euclid(rhs.into_inner()) {
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                #[doc = concat!(
                    "Calculates the remainder of two scalar values, returning `None` ",
                    "on division by zero or primitive signed overflow.\n\n",
                    "Signed overflow can occur for `MIN % -1`.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_rem(", stringify!($name), "::new(3)), Some(", stringify!($name), "::new(1)));\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_rem(", stringify!($name), "::ZERO), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                    match self.into_inner().checked_rem(rhs.into_inner()) {
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                #[doc = concat!(
                    "Calculates the least nonnegative remainder, returning `None` ",
                    "on division by zero or primitive signed overflow.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_rem_euclid(", stringify!($name), "::new(3)), Some(", stringify!($name), "::new(1)));\n",
                    "assert_eq!(", stringify!($name), "::new(10).checked_rem_euclid(", stringify!($name), "::ZERO), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                    match self.into_inner().checked_rem_euclid(rhs.into_inner()) {
                        Some(v) => Some(Self::new(v)),
                        None => None,
                    }
                }

                #[doc = concat!(
                    "Raises `self` to the power of `exp`, saturating at numeric bounds.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(2).pow(3), ", stringify!($name), "::new(8));\n",
                    "assert_eq!(", stringify!($name), "::MAX.pow(2), ", stringify!($name), "::MAX);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn pow(self, exp: u32) -> Self {
                    Self::new(self.into_inner().saturating_pow(exp))
                }

                #[doc = concat!(
                    "Returns the base-`base` logarithm, or `None` if the logarithm is undefined.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(8).checked_ilog(2), Some(3));\n",
                    "assert_eq!(", stringify!($name), "::ZERO.checked_ilog(2), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_ilog(self, base: $inner) -> Option<u32> {
                    self.into_inner().checked_ilog(base)
                }

                #[doc = concat!(
                    "Returns the base-2 logarithm, or `None` if the logarithm is undefined.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(8).checked_ilog2(), Some(3));\n",
                    "assert_eq!(", stringify!($name), "::ZERO.checked_ilog2(), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_ilog2(self) -> Option<u32> {
                    self.into_inner().checked_ilog2()
                }

                #[doc = concat!(
                    "Returns the base-10 logarithm, or `None` if the logarithm is undefined.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert_eq!(", stringify!($name), "::new(100).checked_ilog10(), Some(2));\n",
                    "assert_eq!(", stringify!($name), "::ZERO.checked_ilog10(), None);\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn checked_ilog10(self) -> Option<u32> {
                    self.into_inner().checked_ilog10()
                }

                #[doc = concat!(
                    "Returns `true` if `self` is the minimum representable value.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert!(", stringify!($name), "::MIN.is_min());\n",
                    "assert!(!", stringify!($name), "::MAX.is_min());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn is_min(self) -> bool {
                    self.into_inner() == <$inner>::MIN
                }

                #[doc = concat!(
                    "Returns `true` if `self` is the maximum representable value.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert!(", stringify!($name), "::MAX.is_max());\n",
                    "assert!(!", stringify!($name), "::ZERO.is_max());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn is_max(self) -> bool {
                    self.into_inner() == <$inner>::MAX
                }

                #[doc = concat!(
                    "Returns `true` if `self` is zero.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert!(", stringify!($name), "::ZERO.is_zero());\n",
                    "assert!(!", stringify!($name), "::ONE.is_zero());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn is_zero(self) -> bool {
                    self.into_inner() == 0
                }

                #[doc = concat!(
                    "Returns `true` if `self` is one.\n\n",
                    "# Examples\n\n",
                    "```rust\n",
                    "use satint::", stringify!($name), ";\n\n",
                    "assert!(", stringify!($name), "::ONE.is_one());\n",
                    "assert!(!", stringify!($name), "::ZERO.is_one());\n",
                    "```"
                )]
                #[inline]
                #[must_use]
                pub const fn is_one(self) -> bool {
                    self.into_inner() == 1
                }
            }

            #[doc = concat!(
                "Creates a [`", stringify!($name), "`] from an inner [`",
                stringify!($inner), "`] value.\n\n",
                "# Examples\n\n",
                "```rust\n",
                "use satint::{", stringify!($name), ", ", stringify!($const), "};\n\n",
                "assert_eq!(", stringify!($const), "(42), ", stringify!($name), "::new(42));\n",
                "```"
            )]
            #[inline]
            #[must_use]
            pub const fn $const(value: $inner) -> $name {
                $name::new(value)
            }
        )+
    };
}

pub(crate) use generate_saturating_wrapper;

// let value: i8 = 42;
// let value: Si16 = value.into();
macro_rules! generate_from_primitive_to_wrapper {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl From<$primitive> for $name {
                #[inline]
                fn from(value: $primitive) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new(value as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_from_primitive_to_wrapper;

// let value = si8(42);
// let value: i16 = value.into();
macro_rules! generate_from_wrapper_to_primitive {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl From<$name> for $primitive {
                #[inline]
                fn from(value: $name) -> Self {
                    value.0.0 as $primitive
                }
            }
        )+
    };
}

pub(crate) use generate_from_wrapper_to_primitive;

// let value: i16 = 42;
// let value: Si8 = value.saturating_into();
// We use i128/u128 below since isize/usize has a variable size depending on the platform.

// Signed primitive -> signed wrapper: clamp via i128.
macro_rules! generate_saturating_from_signed_primitive_to_signed_wrapper {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$primitive> for $name {
                #[inline]
                fn saturating_from(value: $primitive) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new((value as i128).clamp(InnerTy::MIN as i128, InnerTy::MAX as i128) as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_signed_primitive_to_signed_wrapper;

// Unsigned primitive -> unsigned wrapper: clamp via u128.
macro_rules! generate_saturating_from_unsigned_primitive_to_unsigned_wrapper {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$primitive> for $name {
                #[inline]
                fn saturating_from(value: $primitive) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new((value as u128).min(InnerTy::MAX as u128) as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_unsigned_primitive_to_unsigned_wrapper;

// Unsigned primitive -> signed wrapper: source is non-negative, clamp upper at dest MAX via u128.
macro_rules! generate_saturating_from_unsigned_primitive_to_signed_wrapper {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$primitive> for $name {
                #[inline]
                fn saturating_from(value: $primitive) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new((value as u128).min(InnerTy::MAX as u128) as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_unsigned_primitive_to_signed_wrapper;

// Signed primitive -> unsigned wrapper: negatives saturate to 0, then clamp upper via u128.
macro_rules! generate_saturating_from_signed_primitive_to_unsigned_wrapper {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$primitive> for $name {
                #[inline]
                fn saturating_from(value: $primitive) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    if value < 0 {
                        Self::new(0)
                    } else {
                        Self::new((value as u128).min(InnerTy::MAX as u128) as InnerTy)
                    }
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_signed_primitive_to_unsigned_wrapper;

// let value = si16(42);
// let value: i8 = value.saturating_into();
macro_rules! generate_saturating_from_wrapper_to_primitive {
    ($name:ident; $as_ty:ty; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$name> for $primitive {
                #[inline]
                fn saturating_from(value: $name) -> Self {
                    (value.into_inner() as $as_ty).clamp(<$primitive>::MIN as $as_ty, <$primitive>::MAX as $as_ty) as $primitive
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_wrapper_to_primitive;

// let value = si16(-5);
// let value: u8 = value.saturating_into();
// Cross-sign: signed wrapper -> unsigned primitive. Negatives saturate to 0,
// values above the destination MAX saturate to that MAX.
macro_rules! generate_saturating_from_signed_wrapper_to_unsigned_primitive {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$name> for $primitive {
                #[inline]
                fn saturating_from(value: $name) -> Self {
                    let value = value.into_inner();
                    if value < 0 {
                        0
                    } else {
                        (value as u128).min(<$primitive>::MAX as u128) as $primitive
                    }
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_signed_wrapper_to_unsigned_primitive;

// let value = su16(300);
// let value: i8 = value.saturating_into();
// Cross-sign: unsigned wrapper -> signed primitive. Source is non-negative,
// values above the destination MAX saturate to that MAX.
macro_rules! generate_saturating_from_unsigned_wrapper_to_signed_primitive {
    ($name:ident; $($primitive:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$name> for $primitive {
                #[inline]
                fn saturating_from(value: $name) -> Self {
                    let value = value.into_inner();
                    (value as u128).min(<$primitive>::MAX as u128) as $primitive
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_unsigned_wrapper_to_signed_primitive;

// let value: Si8 = si8(42);
// let value: Si16 = value.into();
macro_rules! generate_from_wrapper_to_wrapper {
    ($name:ident; $($wrapper:ty),+ $(,)?) => {
        $(
            impl From<$wrapper> for $name {
                #[inline]
                fn from(value: $wrapper) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new(value.into_inner() as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_from_wrapper_to_wrapper;

// let value = si16(42);
// let value: Si8 = value.saturating_into();
macro_rules! generate_saturating_from_wrapper_to_wrapper {
    ($name:ident; $as_ty:ty; $($wrapper:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$wrapper> for $name {
                #[inline]
                fn saturating_from(value: $wrapper) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new((value.into_inner() as $as_ty).clamp(InnerTy::MIN as $as_ty, InnerTy::MAX as $as_ty) as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_wrapper_to_wrapper;

// let value: f16 = 42.0;
// let value: Si8 = value.saturating_into();
macro_rules! generate_saturating_from_float_to_wrapper {
    ($name:ident; $($float:ty),+ $(,)?) => {
        $(
            impl SaturatingFrom<$float> for $name {
                #[inline]
                fn saturating_from(value: $float) -> Self {
                    type InnerTy = <$name as Inner>::Inner;
                    Self::new(value as InnerTy)
                }
            }
        )+
    };
}

pub(crate) use generate_saturating_from_float_to_wrapper;

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        common::{SaturatingFrom, SaturatingInto},
        si::{Si8, Si16, Si32, Si64, Si128, si8, si16, si32, si64, si128},
        su::{Su8, Su16, Su32, Su64, Su128, su8, su16, su32, su64, su128},
    };

    #[test]
    fn saturating_into_blanket() {
        let v: Su8 = (-1i8).saturating_into();
        assert_eq!(v, Su8::ZERO);
        let v: Si8 = u128::MAX.saturating_into();
        assert_eq!(v, Si8::MAX);
        // SaturatingFrom<T> for T identity impl.
        let v: u32 = 42u32.saturating_into();
        assert_eq!(v, 42);
    }

    #[test]
    fn const_constructors() {
        assert_eq!(su8(42), Su8::new(42));
        assert_eq!(su16(42), Su16::new(42));
        assert_eq!(su32(42), Su32::new(42));
        assert_eq!(su64(42), Su64::new(42));
        assert_eq!(su128(42), Su128::new(42));
        assert_eq!(si8(-5), Si8::new(-5));
        assert_eq!(si16(-5), Si16::new(-5));
        assert_eq!(si32(-5), Si32::new(-5));
        assert_eq!(si64(-5), Si64::new(-5));
        assert_eq!(si128(-5), Si128::new(-5));
    }

    #[test]
    fn pow_method() {
        assert_eq!(Su8::new(2).pow(3), Su8::new(8));
        assert_eq!(Su8::new(16).pow(3), Su8::MAX);
        assert_eq!(Si8::new(2).pow(3), Si8::new(8));
        assert_eq!(Si8::new(-2).pow(3), Si8::new(-8));
        assert_eq!(Si8::new(-3).pow(5), Si8::MIN);
    }

    #[test]
    fn from_inner_and_back() {
        // From<inner> for wrapper.
        assert_eq!(Su8::from(42u8), Su8::new(42));
        let s: Si8 = (-5i8).into();
        assert_eq!(s, Si8::new(-5));
        // From<wrapper> for inner.
        let v: u8 = Su8::new(42).into();
        assert_eq!(v, 42);
        let v: i8 = Si8::new(-5).into();
        assert_eq!(v, -5);
    }

    #[test]
    fn from_str_parsing() {
        let s: Su8 = "42".parse().unwrap();
        assert_eq!(s, Su8::new(42));
        let s: Si8 = "-5".parse().unwrap();
        assert_eq!(s, Si8::new(-5));
        assert!("not_a_number".parse::<Su8>().is_err());
        assert!("256".parse::<Su8>().is_err());
    }

    #[test]
    fn debug_and_display_format() {
        assert_eq!(format!("{:?}", Su8::new(42)), "Su8(42)");
        assert_eq!(format!("{:?}", Si16::new(-5)), "Si16(-5)");
        assert_eq!(format!("{}", Su8::new(42)), "42");
        assert_eq!(format!("{}", Si16::new(-5)), "-5");
    }

    #[test]
    fn type_constants() {
        assert_eq!(Su8::BITS, 8);
        assert_eq!(Si128::BITS, 128);
        assert_eq!(Su8::default(), Su8::ZERO);
        assert_eq!(Si16::default(), Si16::ZERO);
    }

    #[test]
    fn copy_clone_ord() {
        let a = Su8::new(42);
        let b = a;
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert!(Su8::new(1) < Su8::new(2));
        assert!(Si8::new(-1) < Si8::new(0));
    }

    #[test]
    fn from_primitive_to_wrapper() {
        let s: Su16 = 42u8.into();
        assert_eq!(s, Su16::new(42));
        let s: Si32 = (-5i16).into();
        assert_eq!(s, Si32::new(-5));
        let s: Si16 = 42u8.into();
        assert_eq!(s, Si16::new(42));
    }

    #[test]
    fn from_wrapper_to_primitive() {
        let v: u32 = Su8::new(42).into();
        assert_eq!(v, 42);
        let v: i32 = Si8::new(-5).into();
        assert_eq!(v, -5);
        let v: f32 = Su8::new(42).into();
        assert_eq!(v, 42.0);
    }

    #[test]
    fn saturating_signed_primitive_to_signed_wrapper() {
        assert_eq!(Si8::saturating_from(42i32), Si8::new(42));
        assert_eq!(Si8::saturating_from(1000i32), Si8::MAX);
        assert_eq!(Si8::saturating_from(-1000i32), Si8::MIN);
        assert_eq!(Si16::saturating_from(-100i32), Si16::new(-100));
    }

    #[test]
    fn saturating_unsigned_primitive_to_unsigned_wrapper() {
        assert_eq!(Su8::saturating_from(42u32), Su8::new(42));
        assert_eq!(Su8::saturating_from(1000u32), Su8::MAX);
        assert_eq!(Su16::saturating_from(65535u32), Su16::MAX);
    }

    #[test]
    fn saturating_unsigned_primitive_to_signed_wrapper() {
        assert_eq!(Si8::saturating_from(42u32), Si8::new(42));
        assert_eq!(Si8::saturating_from(200u32), Si8::MAX);
        assert_eq!(Si8::saturating_from(127u32), Si8::MAX);
    }

    #[test]
    fn saturating_signed_primitive_to_unsigned_wrapper() {
        // Negative branch.
        assert_eq!(Su8::saturating_from(-1i32), Su8::ZERO);
        assert_eq!(Su8::saturating_from(-1000i32), Su8::ZERO);
        // Positive branches.
        assert_eq!(Su8::saturating_from(0i32), Su8::ZERO);
        assert_eq!(Su8::saturating_from(42i32), Su8::new(42));
        assert_eq!(Su8::saturating_from(1000i32), Su8::MAX);
    }

    #[test]
    fn saturating_wrapper_to_primitive() {
        let v: u8 = u8::saturating_from(Su16::new(100));
        assert_eq!(v, 100);
        let v: u8 = u8::saturating_from(Su16::new(1000));
        assert_eq!(v, u8::MAX);
        let v: i8 = i8::saturating_from(Si16::new(50));
        assert_eq!(v, 50);
        let v: i8 = i8::saturating_from(Si16::new(1000));
        assert_eq!(v, i8::MAX);
        let v: i8 = i8::saturating_from(Si16::new(-1000));
        assert_eq!(v, i8::MIN);
    }

    #[test]
    fn saturating_signed_wrapper_to_unsigned_primitive() {
        // Negative branch saturates to 0.
        let v: u8 = u8::saturating_from(Si16::new(-1));
        assert_eq!(v, 0);
        let v: u8 = u8::saturating_from(Si16::MIN);
        assert_eq!(v, 0);
        // In-range pass-through.
        let v: u8 = u8::saturating_from(Si16::new(42));
        assert_eq!(v, 42);
        // Above dest MAX saturates to dest MAX.
        let v: u8 = u8::saturating_from(Si16::new(300));
        assert_eq!(v, u8::MAX);
        // u128 destination: source range fits, no clamp.
        let v: u128 = u128::saturating_from(Si128::MAX);
        assert_eq!(v, i128::MAX as u128);
        let v: u128 = u128::saturating_from(Si128::MIN);
        assert_eq!(v, 0);
    }

    #[test]
    fn saturating_unsigned_wrapper_to_signed_primitive() {
        // In-range pass-through.
        let v: i8 = i8::saturating_from(Su16::new(42));
        assert_eq!(v, 42);
        // Above signed dest MAX saturates.
        let v: i8 = i8::saturating_from(Su16::new(200));
        assert_eq!(v, i8::MAX);
        let v: i8 = i8::saturating_from(Su16::MAX);
        assert_eq!(v, i8::MAX);
        // i128 destination: u128 source can overflow signed MAX.
        let v: i128 = i128::saturating_from(Su128::MAX);
        assert_eq!(v, i128::MAX);
        let v: i128 = i128::saturating_from(Su128::new(42));
        assert_eq!(v, 42);
    }

    #[test]
    fn from_wrapper_to_wrapper_lossless() {
        let s: Su16 = Su8::new(42).into();
        assert_eq!(s, Su16::new(42));
        let s: Si32 = Si8::new(-5).into();
        assert_eq!(s, Si32::new(-5));
    }

    #[test]
    fn saturating_wrapper_to_wrapper_same_sign() {
        assert_eq!(Su8::saturating_from(Su16::new(50)), Su8::new(50));
        assert_eq!(Su8::saturating_from(Su16::new(1000)), Su8::MAX);
        assert_eq!(Si8::saturating_from(Si16::new(50)), Si8::new(50));
        assert_eq!(Si8::saturating_from(Si16::new(1000)), Si8::MAX);
        assert_eq!(Si8::saturating_from(Si16::new(-1000)), Si8::MIN);
    }

    #[test]
    fn saturating_float_to_wrapper() {
        assert_eq!(Su8::saturating_from(42.5_f32), Su8::new(42));
        assert_eq!(Su8::saturating_from(1000.0_f32), Su8::MAX);
        assert_eq!(Su8::saturating_from(-1.0_f32), Su8::ZERO);
        assert_eq!(Si8::saturating_from(42.5_f64), Si8::new(42));
        assert_eq!(Si8::saturating_from(1000.0_f64), Si8::MAX);
        assert_eq!(Si8::saturating_from(-1000.0_f64), Si8::MIN);
    }

    #[test]
    fn cross_eq_and_ord_with_inner() {
        let w = Su8::new(42);
        // wrapper <op> inner
        assert!(w == 42u8);
        assert!(w != 41u8);
        assert!(w > 1u8);
        assert!(w < 100u8);
        assert_eq!(w.partial_cmp(&42u8), Some(core::cmp::Ordering::Equal));
        // inner <op> wrapper
        assert!(42u8 == w);
        assert!(41u8 != w);
        assert!(1u8 < w);
        assert_eq!(42u8.partial_cmp(&w), Some(core::cmp::Ordering::Equal));
    }

    #[test]
    fn sum_and_product_iterators() {
        let owned = [Su8::new(1), Su8::new(2), Su8::new(3)];
        let s: Su8 = owned.iter().copied().sum();
        assert_eq!(s, Su8::new(6));
        let s_ref: Su8 = owned.iter().sum();
        assert_eq!(s_ref, Su8::new(6));

        let p: Su8 = owned.iter().copied().product();
        assert_eq!(p, Su8::new(6));
        let p_ref: Su8 = owned.iter().product();
        assert_eq!(p_ref, Su8::new(6));

        // Empty iterator yields ZERO for sum and ONE for product.
        let empty: [Su8; 0] = [];
        let s: Su8 = empty.iter().copied().sum();
        assert_eq!(s, Su8::ZERO);
        let p: Su8 = empty.iter().copied().product();
        assert_eq!(p, Su8::ONE);
    }

    #[test]
    fn bitwise_ops_with_self() {
        let a = Su8::new(0b1100);
        let b = Su8::new(0b1010);
        assert_eq!(a & b, Su8::new(0b1000));
        assert_eq!(a | b, Su8::new(0b1110));
        assert_eq!(a ^ b, Su8::new(0b0110));
        assert_eq!(!Su8::new(0), Su8::MAX);

        let mut x = a;
        x &= b;
        assert_eq!(x, Su8::new(0b1000));
        let mut x = a;
        x |= b;
        assert_eq!(x, Su8::new(0b1110));
        let mut x = a;
        x ^= b;
        assert_eq!(x, Su8::new(0b0110));
    }

    #[test]
    fn bitwise_ops_with_inner() {
        let a = Su8::new(0b1100);
        assert_eq!(a & 0b1010u8, Su8::new(0b1000));
        assert_eq!(a | 0b1010u8, Su8::new(0b1110));
        assert_eq!(a ^ 0b1010u8, Su8::new(0b0110));

        let mut x = a;
        x &= 0b1010u8;
        assert_eq!(x, Su8::new(0b1000));
        let mut x = a;
        x |= 0b1010u8;
        assert_eq!(x, Su8::new(0b1110));
        let mut x = a;
        x ^= 0b1010u8;
        assert_eq!(x, Su8::new(0b0110));
    }

    #[test]
    fn bit_inspection_methods() {
        let v = Su8::new(0b1011_0100);
        assert_eq!(v.count_ones(), 4);
        assert_eq!(v.count_zeros(), 4);
        assert_eq!(v.leading_zeros(), 0);
        assert_eq!(Su8::new(0b0000_1111).leading_zeros(), 4);
        assert_eq!(v.leading_ones(), 1);
        assert_eq!(Su8::new(0b1111_0000).leading_ones(), 4);
        assert_eq!(v.trailing_zeros(), 2);
        assert_eq!(Su8::new(0b0000_0111).trailing_ones(), 3);
        assert_eq!(v.trailing_ones(), 0);
    }

    #[test]
    fn bit_transform_methods() {
        assert_eq!(Su8::new(0b0000_0001).reverse_bits(), Su8::new(0b1000_0000));
        assert_eq!(Su8::new(0b0000_0001).rotate_left(1), Su8::new(0b0000_0010));
        assert_eq!(Su8::new(0b0000_0001).rotate_right(1), Su8::new(0b1000_0000));
        assert_eq!(Su16::new(0x00FF).swap_bytes(), Su16::new(0xFF00));
    }

    #[test]
    fn endian_conversions() {
        let v = Su16::new(0x1234);
        // Round-trip through both endians.
        assert_eq!(Su16::from_be(v.to_be()), v);
        assert_eq!(Su16::from_le(v.to_le()), v);

        let be_bytes = v.to_be_bytes();
        let le_bytes = v.to_le_bytes();
        let ne_bytes = v.to_ne_bytes();
        assert_eq!(be_bytes, [0x12, 0x34]);
        assert_eq!(le_bytes, [0x34, 0x12]);
        assert_eq!(Su16::from_be_bytes(be_bytes), v);
        assert_eq!(Su16::from_le_bytes(le_bytes), v);
        assert_eq!(Su16::from_ne_bytes(ne_bytes), v);
    }

    #[test]
    fn checked_div_and_rem() {
        // Normal division.
        assert_eq!(Si8::new(10).checked_div(Si8::new(3)), Some(Si8::new(3)));
        assert_eq!(Si8::new(10).checked_rem(Si8::new(3)), Some(Si8::new(1)));
        // Division by zero returns None.
        assert_eq!(Si8::new(10).checked_div(Si8::ZERO), None);
        assert_eq!(Si8::new(10).checked_rem(Si8::ZERO), None);
        // Signed overflow (MIN / -1, MIN % -1) returns None.
        assert_eq!(Si8::MIN.checked_div(Si8::new(-1)), None);
        assert_eq!(Si8::MIN.checked_rem(Si8::new(-1)), None);

        // Euclidean variants.
        assert_eq!(
            Si8::new(-7).checked_div_euclid(Si8::new(3)),
            Some(Si8::new(-3)),
        );
        assert_eq!(
            Si8::new(-7).checked_rem_euclid(Si8::new(3)),
            Some(Si8::new(2)),
        );
        assert_eq!(Si8::new(10).checked_div_euclid(Si8::ZERO), None);
        assert_eq!(Si8::new(10).checked_rem_euclid(Si8::ZERO), None);
        assert_eq!(Si8::MIN.checked_div_euclid(Si8::new(-1)), None);
        assert_eq!(Si8::MIN.checked_rem_euclid(Si8::new(-1)), None);
    }

    #[test]
    fn checked_ilog_methods() {
        assert_eq!(Su8::new(8).checked_ilog(2), Some(3));
        assert_eq!(Su8::new(0).checked_ilog(2), None);
        assert_eq!(Su8::new(8).checked_ilog2(), Some(3));
        assert_eq!(Su8::new(0).checked_ilog2(), None);
        assert_eq!(Su8::new(100).checked_ilog10(), Some(2));
        assert_eq!(Su8::new(0).checked_ilog10(), None);
    }

    #[test]
    fn is_predicates() {
        assert!(Su8::MIN.is_min());
        assert!(!Su8::new(1).is_min());
        assert!(Su8::MAX.is_max());
        assert!(!Su8::new(1).is_max());
        assert!(Su8::ZERO.is_zero());
        assert!(!Su8::ONE.is_zero());
        assert!(Su8::ONE.is_one());
        assert!(!Su8::ZERO.is_one());
        // Signed wrapper exercises distinct MIN/MAX values.
        assert!(Si8::MIN.is_min());
        assert!(Si8::MAX.is_max());
    }
}
