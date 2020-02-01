use std::cmp::Ordering;

use strum::IntoEnumIterator;
use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// LoL and TFT rank divisions, I, II, III, IV, and (deprecated) V.
///
/// Ordered such that "higher" divisions are greater than "lower" ones: `Division::I > Division::IV`.
///
/// Repr'd as equivalent numeric values, (1, 2, 3, 4, 5).
///
/// Implements [IntoEnumIterator](super::IntoEnumIterator). Iterator excludes deprecated `Division::V`.
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Division {
    I   = 1,
    II  = 2,
    III = 3,
    IV  = 4,
    #[deprecated(note="Removed for 2019.")]
    V   = 5,
}

serde_string!(Division);

/// Returns a DoubleEndedIterator of I, II, III, IV.
/// Ordered from high rank (I) to low (IV).
/// Excludes V, which is deprecated.
impl IntoEnumIterator for Division {
    type Iterator = std::slice::Iter<'static, Self>;
    fn iter() -> Self::Iterator {
        [ Self::I, Self::II, Self::III, Self::IV ].iter()
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