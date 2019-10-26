#![allow(deprecated)]

use std::fmt::Debug;

use strum_macros::{ EnumString, Display, AsRefStr };

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(EnumString, Display, AsRefStr)]
#[repr(u8)]
pub enum Tier {
    #[strum(to_string="IRON")]        Iron        =  40,
    #[strum(to_string="BRONZE")]      Bronze      =  60,
    #[strum(to_string="SILVER")]      Silver      =  80,
    #[strum(to_string="GOLD")]        Gold        = 100,
    #[strum(to_string="PLATINUM")]    Platinum    = 120,
    #[strum(to_string="DIAMOND")]     Diamond     = 140,
    #[strum(to_string="MASTER")]      Master      = 200,
    #[strum(to_string="GRANDMASTER")] Grandmaster = 220,
    #[strum(to_string="CHALLENGER")]  Challenger  = 240,
}

impl Tier {
    /// If this tier is "standard".
    /// Standard means non-apex (not master+), and not unranked.
    ///
    /// Only these tiers are queryable by LeagueV4Endpoints::get_league_entries(...).
    pub fn is_standard_tier(self) -> bool {
        self < Self::Master
    }

    /// If this tier is an apex tier.
    /// Master and above.
    pub fn is_apex_tier(self) -> bool {
        Self::Master <= self
    }
}
