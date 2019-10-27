#![allow(deprecated)]

use strum_macros::{ EnumString, Display, AsRefStr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// LoL and TFT ranked tiers, such as gold, diamond, challenger, etc.
///
/// Sorts from lowest rank to highest rank.
///
/// Repr'd as arbitrary u8 values.
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(EnumString, Display, AsRefStr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Tier {
    IRON        =  40,
    BRONZE      =  60,
    SILVER      =  80,
    GOLD        = 100,
    PLATINUM    = 120,
    DIAMOND     = 140,
    MASTER      = 180,
    GRANDMASTER = 200,
    CHALLENGER  = 220,
}

serde_string!(Tier);

impl Tier {
    /// If this tier is "standard".
    /// Standard means non-apex (not master+), and not unranked.
    ///
    /// Only these tiers are queryable by LeagueV4Endpoints::get_league_entries(...).
    pub fn is_standard_tier(self) -> bool {
        self < Self::MASTER
    }

    /// If this tier is an apex tier.
    /// Master and above.
    pub fn is_apex_tier(self) -> bool {
        Self::MASTER <= self
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
    fn to_string() {
        assert_eq!("GRANDMASTER", Tier::GRANDMASTER.as_ref());
        assert_eq!("GRANDMASTER", Tier::GRANDMASTER.to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Ok(Tier::GRANDMASTER), "GRANDMASTER".parse());
    }
}
