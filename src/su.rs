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
