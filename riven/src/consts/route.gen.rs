// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version: 996d171a2b79e9bb85c549f47b07c6ef2721fc8a

use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};
///Regional routes, used in tournament services, Legends of Runeterra (LoR), and some other endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[repr(u8)]
#[non_exhaustive]
pub enum RegionalRoute {
    ///North and South America.
    ///
    ///`1` (riotapi-schema ID/repr)
    #[strum(to_string = "AMERICAS")]
    AMERICAS = 1,
    ///Asia, used for LoL matches (`match-v5`) and TFT matches (`tft-match-v1`).
    ///
    ///`2` (riotapi-schema ID/repr)
    #[strum(to_string = "ASIA")]
    ASIA = 2,
    ///Europe.
    ///
    ///`3` (riotapi-schema ID/repr)
    #[strum(to_string = "EUROPE")]
    EUROPE = 3,
    ///South East Asia, used for LoR, LoL matches (`match-v5`), and TFT matches (`tft-match-v1`).
    ///
    ///`4` (riotapi-schema ID/repr)
    #[strum(to_string = "SEA")]
    SEA = 4,
    ///Asia-Pacific, deprecated, for some old matches in `lor-match-v1`.
    ///
    ///`10` (riotapi-schema ID/repr)
    #[deprecated]
    #[strum(to_string = "APAC")]
    APAC = 10,
    ///Special esports platform for `account-v1`. Do not confuse with the `esports` Valorant platform route.
    ///
    ///`11` (riotapi-schema ID/repr)
    #[strum(to_string = "ESPORTS")]
    ESPORTS = 11,
    ///Special Europe esports platform for `account-v1`. Do not confuse with the `esports` Valorant platform route.
    ///
    ///`12` (riotapi-schema ID/repr)
    #[strum(to_string = "ESPORTSEU")]
    ESPORTSEU = 12,
}
///Platform routes for League of Legends (LoL), Teamfight Tactics (TFT), and Legends of Runeterra (LoR).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[repr(u8)]
#[non_exhaustive]
pub enum PlatformRoute {
    ///Brazil.
    ///
    ///`16` (riotapi-schema ID/repr)
    #[strum(to_string = "BR1", serialize = "BR")]
    BR1 = 16,
    ///Europe, Northeast.
    ///
    ///`17` (riotapi-schema ID/repr)
    #[strum(to_string = "EUN1", serialize = "EUNE")]
    EUN1 = 17,
    ///Europe, West.
    ///
    ///`18` (riotapi-schema ID/repr)
    #[strum(to_string = "EUW1", serialize = "EUW")]
    EUW1 = 18,
    ///Japan.
    ///
    ///`19` (riotapi-schema ID/repr)
    #[strum(to_string = "JP1", serialize = "JP")]
    JP1 = 19,
    ///Korea.
    ///
    ///`20` (riotapi-schema ID/repr)
    #[strum(to_string = "KR")]
    KR = 20,
    ///Latin America, North.
    ///
    ///`21` (riotapi-schema ID/repr)
    #[strum(to_string = "LA1", serialize = "LAN")]
    LA1 = 21,
    ///Latin America, South.
    ///
    ///`22` (riotapi-schema ID/repr)
    #[strum(to_string = "LA2", serialize = "LAS")]
    LA2 = 22,
    ///Middle East and North Africa.
    ///
    ///`37` (riotapi-schema ID/repr)
    #[strum(to_string = "ME1", serialize = "MENA")]
    ME1 = 37,
    ///North America.
    ///
    ///`23` (riotapi-schema ID/repr)
    #[strum(to_string = "NA1", serialize = "NA")]
    NA1 = 23,
    ///Oceania.
    ///
    ///`24` (riotapi-schema ID/repr)
    #[strum(to_string = "OC1", serialize = "OCE")]
    OC1 = 24,
    ///Philippines, moved into `sg2` on 2025-01-08.
    ///
    ///`32` (riotapi-schema ID/repr)
    #[deprecated]
    #[strum(to_string = "PH2", serialize = "PH")]
    PH2 = 32,
    ///Russia
    ///
    ///`25` (riotapi-schema ID/repr)
    #[strum(to_string = "RU")]
    RU = 25,
    ///Singapore, Thailand, Philippines
    ///
    ///`33` (riotapi-schema ID/repr)
    #[strum(to_string = "SG2", serialize = "SG")]
    SG2 = 33,
    ///Thailand, moved into `sg2` on 2025-01-08.
    ///
    ///`34` (riotapi-schema ID/repr)
    #[deprecated]
    #[strum(to_string = "TH2", serialize = "TH")]
    TH2 = 34,
    ///Turkey
    ///
    ///`26` (riotapi-schema ID/repr)
    #[strum(to_string = "TR1", serialize = "TR")]
    TR1 = 26,
    ///Taiwan
    ///
    ///`35` (riotapi-schema ID/repr)
    #[strum(to_string = "TW2", serialize = "TW")]
    TW2 = 35,
    ///Vietnam
    ///
    ///`36` (riotapi-schema ID/repr)
    #[strum(to_string = "VN2", serialize = "VN")]
    VN2 = 36,
    ///Public Beta Environment, special beta testing platform. Located in North America.
    ///
    ///`31` (riotapi-schema ID/repr)
    #[strum(to_string = "PBE1", serialize = "PBE")]
    PBE1 = 31,
}
impl PlatformRoute {
    /// Converts this [`PlatformRoute`] into its corresponding
    /// [`RegionalRoute`] for LoL and TFT match endpoints such as
    /// [`match-v5`](crate::endpoints::MatchV5).
    pub fn to_regional(self) -> RegionalRoute {
        match self {
            Self::BR1 => RegionalRoute::AMERICAS,
            Self::EUN1 => RegionalRoute::EUROPE,
            Self::EUW1 => RegionalRoute::EUROPE,
            Self::JP1 => RegionalRoute::ASIA,
            Self::KR => RegionalRoute::ASIA,
            Self::LA1 => RegionalRoute::AMERICAS,
            Self::LA2 => RegionalRoute::AMERICAS,
            Self::ME1 => RegionalRoute::EUROPE,
            Self::NA1 => RegionalRoute::AMERICAS,
            Self::OC1 => RegionalRoute::SEA,
            Self::PH2 => RegionalRoute::SEA,
            Self::RU => RegionalRoute::EUROPE,
            Self::SG2 => RegionalRoute::SEA,
            Self::TH2 => RegionalRoute::SEA,
            Self::TR1 => RegionalRoute::EUROPE,
            Self::TW2 => RegionalRoute::SEA,
            Self::VN2 => RegionalRoute::SEA,
            Self::PBE1 => RegionalRoute::AMERICAS,
        }
    }
    /// Converts this [`PlatformRoute`] into its corresponding
    /// [`RegionalRoute`] for LoR endpoints such as
    /// [`lor-match-v1`](crate::endpoints::LorMatchV1).
    pub fn to_regional_lor(self) -> RegionalRoute {
        match self {
            Self::BR1 => RegionalRoute::AMERICAS,
            Self::EUN1 => RegionalRoute::EUROPE,
            Self::EUW1 => RegionalRoute::EUROPE,
            Self::JP1 => RegionalRoute::ASIA,
            Self::KR => RegionalRoute::ASIA,
            Self::LA1 => RegionalRoute::AMERICAS,
            Self::LA2 => RegionalRoute::AMERICAS,
            Self::ME1 => RegionalRoute::EUROPE,
            Self::NA1 => RegionalRoute::AMERICAS,
            Self::OC1 => RegionalRoute::SEA,
            Self::PH2 => RegionalRoute::SEA,
            Self::RU => RegionalRoute::SEA,
            Self::SG2 => RegionalRoute::SEA,
            Self::TH2 => RegionalRoute::SEA,
            Self::TR1 => RegionalRoute::SEA,
            Self::TW2 => RegionalRoute::SEA,
            Self::VN2 => RegionalRoute::SEA,
            Self::PBE1 => RegionalRoute::AMERICAS,
        }
    }
    /// Used in the LoL Tournament API. Specifically
    /// [`tournament-stub-v5.registerProviderData`](crate::endpoints::TournamentStubV5::register_provider_data)
    /// and [`tournament-v5.registerProviderData`](crate::endpoints::TournamentV5::register_provider_data).
    ///
    /// Returns `None` if the corresponding tournament region is unknown: <https://github.com/MingweiSamuel/riotapi-schema/issues/58>.
    pub fn to_tournament_region(self) -> Option<TournamentRegion> {
        match self {
            Self::BR1 => Some(TournamentRegion::BR),
            Self::EUN1 => Some(TournamentRegion::EUNE),
            Self::EUW1 => Some(TournamentRegion::EUW),
            Self::JP1 => Some(TournamentRegion::JP),
            Self::KR => None,
            Self::LA1 => Some(TournamentRegion::LAN),
            Self::LA2 => Some(TournamentRegion::LAS),
            Self::ME1 => None,
            Self::NA1 => Some(TournamentRegion::NA),
            Self::OC1 => Some(TournamentRegion::OCE),
            Self::PH2 => None,
            Self::RU => None,
            Self::SG2 => None,
            Self::TH2 => None,
            Self::TR1 => Some(TournamentRegion::TR),
            Self::TW2 => None,
            Self::VN2 => None,
            Self::PBE1 => Some(TournamentRegion::PBE),
        }
    }
    /// Get the slightly more human-friendly alternate name for this `PlatformRoute`. Specifically
    /// excludes any trailing numbers and appends extra N(orth), S(outh), E(ast), and/or W(est)
    /// suffixes to some names. Some of these are old region names which are often still used as
    /// user-facing names, e.g. on op.gg.
    ///
    /// Note these strings *are* handled by the `FromStr` implementation, if you wish to parse them
    /// back into `PlatformRoute`s.
    pub fn as_region_str(self) -> &'static str {
        match self {
            Self::BR1 => "BR",
            Self::EUN1 => "EUNE",
            Self::EUW1 => "EUW",
            Self::JP1 => "JP",
            Self::KR => "KR",
            Self::LA1 => "LAN",
            Self::LA2 => "LAS",
            Self::ME1 => "MENA",
            Self::NA1 => "NA",
            Self::OC1 => "OCE",
            Self::PH2 => "PH",
            Self::RU => "RU",
            Self::SG2 => "SG",
            Self::TH2 => "TH",
            Self::TR1 => "TR",
            Self::TW2 => "TW",
            Self::VN2 => "VN",
            Self::PBE1 => "PBE",
        }
    }
}
///Platform routes for Valorant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[repr(u8)]
#[non_exhaustive]
pub enum ValPlatformRoute {
    ///Asia-Pacific.
    ///
    ///`64` (riotapi-schema ID/repr)
    #[strum(to_string = "AP")]
    AP = 64,
    ///Brazil.
    ///
    ///`65` (riotapi-schema ID/repr)
    #[strum(to_string = "BR")]
    BR = 65,
    ///Europe.
    ///
    ///`66` (riotapi-schema ID/repr)
    #[strum(to_string = "EU")]
    EU = 66,
    ///Korea.
    ///
    ///`70` (riotapi-schema ID/repr)
    #[strum(to_string = "KR")]
    KR = 70,
    ///Latin America.
    ///
    ///`68` (riotapi-schema ID/repr)
    #[strum(to_string = "LATAM")]
    LATAM = 68,
    ///North America.
    ///
    ///`69` (riotapi-schema ID/repr)
    #[strum(to_string = "NA")]
    NA = 69,
    ///Special esports platform.
    ///
    ///`95` (riotapi-schema ID/repr)
    #[strum(to_string = "ESPORTS")]
    ESPORTS = 95,
}
/// Tournament regions for League of Legends (LoL) used in
/// [`TournamentStubV5`](crate::endpoints::TournamentStubV5)
/// and [`TournamentV5`](crate::endpoints::TournamentV5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[derive(serde::Serialize, crate::de::Deserialize)]
#[repr(u8)]
#[non_exhaustive]
pub enum TournamentRegion {
    ///Brazil.
    ///
    ///`16` (riotapi-schema ID/repr)
    BR = 16,
    ///Europe, Northeast.
    ///
    ///`17` (riotapi-schema ID/repr)
    EUNE = 17,
    ///Europe, West.
    ///
    ///`18` (riotapi-schema ID/repr)
    EUW = 18,
    ///Japan.
    ///
    ///`19` (riotapi-schema ID/repr)
    JP = 19,
    ///Latin America, North.
    ///
    ///`21` (riotapi-schema ID/repr)
    LAN = 21,
    ///Latin America, South.
    ///
    ///`22` (riotapi-schema ID/repr)
    LAS = 22,
    ///North America.
    ///
    ///`23` (riotapi-schema ID/repr)
    NA = 23,
    ///Oceania.
    ///
    ///`24` (riotapi-schema ID/repr)
    OCE = 24,
    ///Turkey
    ///
    ///`26` (riotapi-schema ID/repr)
    TR = 26,
    ///Public Beta Environment, special beta testing platform. Located in North America.
    ///
    ///`31` (riotapi-schema ID/repr)
    PBE = 31,
}

