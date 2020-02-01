use num_enum::{ IntoPrimitive, TryFromPrimitive };
use strum_macros::{ EnumString, EnumIter, Display, AsRefStr, IntoStaticStr };

/// LoL and TFT ranked tiers, such as gold, diamond, challenger, etc.
///
/// Sorts from lowest rank to highest rank.
///
/// Repr'd as arbitrary `u8` values.
///
/// Implements [IntoEnumIterator](super::IntoEnumIterator).
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, AsRefStr, IntoStaticStr)]
#[repr(u8)]
pub enum Tier {
    /// Challenger, the highest tier, an apex tier. Repr: `220_u8`.
    CHALLENGER  = 220,
    /// Grand Master, an apex tier. Repr: `200_u8`.
    GRANDMASTER = 200,
    /// Master, an apex tier. Repr: `180_u8`.
    MASTER      = 180,
    /// Diamond, the higest non-apex tier. Repr: `140_u8`.
    DIAMOND     = 140,
    /// Platinum. Repr: `120_u8`.
    PLATINUM    = 120,
    /// Gold. Repr: `100_u8`.
    GOLD        = 100,
    /// Silver. Repr: `80_u8`.
    SILVER      =  80,
    /// Bronze. Repr: `60_u8`.
    BRONZE      =  60,
    /// Iron, the lowest tier. Repr: `40_u8`.
    IRON        =  40,
}

serde_string!(Tier);

impl Tier {
    /// If this tier is an apex tier: master and above.
    ///
    /// Inverse of is_standard().
    ///
    /// These tiers are NOT queryable by LeagueV4Endpoints::get_league_entries(...).
    pub const fn is_apex(self) -> bool {
        (Self::MASTER as u8) <= (self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        assert!(Tier::GOLD < Tier::DIAMOND);
    }

    #[test]
    fn apex_check() {
        assert!( Tier::GRANDMASTER.is_apex());
        assert!(!Tier::DIAMOND.is_apex());
    }

    #[test]
    fn to_string() {
        assert_eq!("GRANDMASTER", Tier::GRANDMASTER.as_ref());
        assert_eq!("GRANDMASTER", Tier::GRANDMASTER.to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Ok(Tier::GRANDMASTER), "GRANDMASTER".parse());
    }

    #[test]
    fn iter() {
        use strum::IntoEnumIterator;
        let mut iter = Tier::iter();
        assert_eq!(Some(Tier::CHALLENGER), iter.next());
        iter.next();
        iter.next();
        assert_eq!(Some(Tier::DIAMOND), iter.next());
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(Some(Tier::IRON), iter.next());
        assert_eq!(None, iter.next());
    }
}
