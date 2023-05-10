use std::cmp::Ordering;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// LoL and TFT rank divisions, I, II, III, IV, and (deprecated) V.
///
/// Ordered such that "higher" divisions are greater than "lower" ones: `Division::I > Division::IV`.
///
/// Repr'd as equivalent numeric values, (1, 2, 3, 4, 5).
///
/// Implements [IntoEnumIterator](super::IntoEnumIterator). Iterator excludes deprecated `Division::V`.
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    EnumString,
    Display,
    AsRefStr,
    IntoStaticStr,
    IntoPrimitive,
    TryFromPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Division {
    /// Division 1, the best/highest division in a [`Tier`](crate::consts::Tier), or the only division in
    /// [apex tiers](crate::consts::Tier::is_apex).
    I = 1,
    /// Division 2, the second highest division.
    II = 2,
    /// Division 3, the third highest division.
    III = 3,
    /// Division 4, the fourth and lowest division since 2019.
    IV = 4,
    /// Division 5, the lowest division, only used before 2019.
    #[deprecated(note = "Removed for 2019.")]
    V = 5,
}

/// Returns a DoubleEndedIterator of I, II, III, IV.
/// Ordered from high rank (I) to low (IV).
/// Excludes V, which is deprecated.
impl IntoEnumIterator for Division {
    type Iterator = std::iter::Copied<std::slice::Iter<'static, Self>>;
    fn iter() -> Self::Iterator {
        [Self::I, Self::II, Self::III, Self::IV].iter().copied()
    }
}

impl Ord for Division {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other)).reverse()
    }
}

impl PartialOrd for Division {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        assert!(Division::IV < Division::I);
    }
}
