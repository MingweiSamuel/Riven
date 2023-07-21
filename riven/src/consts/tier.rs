use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// LoL and TFT ranked tiers, such as gold, diamond, challenger, etc.
///
/// Sorts from lowest rank to highest rank.
///
/// Repr'd as arbitrary `u8` values.
///
/// Implements [IntoEnumIterator](super::IntoEnumIterator).
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    TryFromPrimitive,
    EnumString,
    Display,
    AsRefStr,
    IntoStaticStr,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Tier {
    /// Challenger, the highest tier, an apex tier. Repr: `220_u8`.
    CHALLENGER = 220,
    /// Grand Master, an apex tier. Repr: `200_u8`.
    GRANDMASTER = 200,
    /// Master, an apex tier. Repr: `180_u8`.
    MASTER = 180,
    /// Diamond, the higest non-apex tier. Repr: `140_u8`.
    DIAMOND = 140,
    /// Emerald. Added in 2023. Repr: `130_u8`.
    EMERALD = 130,
    /// Platinum. Repr: `120_u8`.
    PLATINUM = 120,
    /// Gold. Repr: `100_u8`.
    GOLD = 100,
    /// Silver. Repr: `80_u8`.
    SILVER = 80,
    /// Bronze. Repr: `60_u8`.
    BRONZE = 60,
    /// Iron, the lowest tier. Repr: `40_u8`.
    IRON = 40,

    /// Unranked, no tier. Repr: `0_u8`.
    /// Also deserializes from "NONE" returned by `lol-challenges-v1.getChallengePercentiles`.
    #[serde(alias = "NONE")]
    UNRANKED = 0,
}

impl Tier {
    /// If this tier is an apex tier: [`Self::MASTER`], [`Self::GRANDMASTER`],
    /// or [`Self::CHALLENGER`]. Returns false for [`Self::UNRANKED`].
    ///
    /// These tiers are NOT queryable by LeagueV4Endpoints::get_league_entries(...).
    pub const fn is_apex(self) -> bool {
        // Casts needed for const.
        (Self::MASTER as u8) <= (self as u8)
    }

    /// If this tier is a "standard" tier: iron through diamond.
    /// Returns false for unranked.
    ///
    /// ONLY these tiers are queryable by [`LeagueV4::get_league_entries(...)`](crate::endpoints::LeagueV4::get_league_entries).
    pub fn is_standard(self) -> bool {
        // Casts needed for const.
        ((Self::UNRANKED as u8) < (self as u8)) && ((self as u8) < (Self::MASTER as u8))
    }

    /// If this tier is ranked. Returns true for iron through challenger, false for unranked.
    pub const fn is_ranked(self) -> bool {
        // Casts needed for const.
        (Self::UNRANKED as u8) < (self as u8)
    }

    /// If this tier is unranked (`Tier::UNRANKED`).
    ///
    /// UNRANKED is returned by `Participant.highest_achieved_season_tier`.
    pub const fn is_unranked(self) -> bool {
        // Casts needed for const.
        (self as u8) <= (Self::UNRANKED as u8)
    }

    /// Converts UNRANKED to None and all ranked tiers to Some(...).
    pub fn to_ranked(self) -> Option<Self> {
        if self.is_unranked() {
            None
        } else {
            Some(self)
        }
    }
}

/// Returns a DoubleEndedIterator of I, II, III, IV.
/// Ordered from high rank (I) to low (IV).
/// Excludes V, which is deprecated.
impl IntoEnumIterator for Tier {
    type Iterator = std::iter::Copied<std::slice::Iter<'static, Self>>;
    fn iter() -> Self::Iterator {
        [
            Self::CHALLENGER,
            Self::GRANDMASTER,
            Self::MASTER,
            Self::DIAMOND,
            Self::EMERALD,
            Self::PLATINUM,
            Self::GOLD,
            Self::SILVER,
            Self::BRONZE,
            Self::IRON,
        ]
        .iter()
        .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord() {
        assert!(Tier::GOLD < Tier::DIAMOND);
        assert!(Tier::UNRANKED < Tier::IRON);
    }

    #[test]
    fn is_apex() {
        assert!(Tier::GRANDMASTER.is_apex());
        assert!(!Tier::DIAMOND.is_apex());
        assert!(!Tier::UNRANKED.is_apex());
    }

    #[test]
    fn is_ranked() {
        assert!(Tier::GRANDMASTER.is_ranked());
        assert!(Tier::DIAMOND.is_ranked());
        assert!(!Tier::UNRANKED.is_ranked());
    }

    #[test]
    fn is_unranked() {
        assert!(!Tier::GRANDMASTER.is_unranked());
        assert!(!Tier::DIAMOND.is_unranked());
        assert!(Tier::UNRANKED.is_unranked());
    }

    #[test]
    fn to_ranked() {
        assert_eq!(Some(Tier::GRANDMASTER), Tier::GRANDMASTER.to_ranked());
        assert_eq!(Some(Tier::DIAMOND), Tier::DIAMOND.to_ranked());
        assert_eq!(None, Tier::UNRANKED.to_ranked());
    }

    #[test]
    fn is_standard() {
        assert!(!Tier::GRANDMASTER.is_standard());
        assert!(Tier::DIAMOND.is_standard());
        assert!(!Tier::UNRANKED.is_standard());
    }

    #[test]
    fn to_string() {
        assert_eq!("GRANDMASTER", Tier::GRANDMASTER.as_ref());
        assert_eq!("GRANDMASTER", Tier::GRANDMASTER.to_string());
        assert_eq!("UNRANKED", Tier::UNRANKED.as_ref());
        assert_eq!("UNRANKED", Tier::UNRANKED.to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Ok(Tier::GRANDMASTER), "GRANDMASTER".parse());
        assert_eq!(Ok(Tier::UNRANKED), "UNRANKED".parse());
    }

    #[test]
    fn iter() {
        use strum::IntoEnumIterator;

        let mut iter = Tier::iter();
        assert_eq!(Some(Tier::CHALLENGER), iter.next());
        iter.next();
        iter.next();
        assert_eq!(Some(Tier::DIAMOND), iter.next());
        assert_eq!(Some(Tier::EMERALD), iter.next());
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(Some(Tier::IRON), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());

        let mut iter = Tier::iter().rev();
        assert_eq!(Some(Tier::IRON), iter.next());
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(Some(Tier::EMERALD), iter.next());
        assert_eq!(Some(Tier::DIAMOND), iter.next());
        iter.next();
        iter.next();
        assert_eq!(Some(Tier::CHALLENGER), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());

        let mut iter = Tier::iter();
        assert_eq!(Some(Tier::CHALLENGER), iter.next());
        assert_eq!(Some(Tier::IRON), iter.next_back());
    }
}
