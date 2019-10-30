use std::cmp::Ordering;

use strum_macros::{ EnumString, Display, AsRefStr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// LoL and TFT rank divisions, I, II, III, IV, and (deprecated) V.
///
/// Sorts in reverse numeric order, from low to high rank.
///
/// Repr'd as equivalent numeric values, (1, 2, 3, 4, 5).
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, Display, AsRefStr)]
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