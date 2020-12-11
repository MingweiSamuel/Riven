use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };

/// A region served by a single game server.
/// Each Riot Games API request is directed at a particular region,
/// with tournament API requests directed at the AMERICAS "global" region.
///
/// Valorant regions are prefixed with `VAL_` due to the name collision with
/// `BR` ("BR1") for LoL and `BR` ("BR") for Valorant.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
#[derive(Clone, Copy)]
pub enum Region {
    #[strum(to_string="BR1", serialize="BR")]
    BR,
    #[strum(to_string="EUN1", serialize="EUNE")]
    EUNE,
    #[strum(to_string="EUW1", serialize="EUW")]
    EUW,
    #[strum(to_string="NA1", serialize="NA")]
    NA,
    #[strum(to_string="KR", serialize="KR")]
    KR,
    #[strum(to_string="LA1", serialize="LAN")]
    LAN,
    #[strum(to_string="LA2", serialize="LAS")]
    LAS,
    #[strum(to_string="OC1", serialize="OCE")]
    OCE,
    #[strum(to_string="RU", serialize="RU")]
    RU,
    #[strum(to_string="TR1", serialize="TR")]
    TR,
    #[strum(to_string="JP1", serialize="JP")]
    JP,
    #[strum(to_string="PBE1", serialize="PBE")]
    PBE,
    #[strum(to_string="AMERICAS", serialize="AMERICAS")]
    AMERICAS,
    #[strum(to_string="EUROPE", serialize="EUROPE")]
    EUROPE,
    #[strum(to_string="ASIA", serialize="ASIA")]
    ASIA,

    #[strum(to_string="AP", serialize="AP")]
    VAL_AP,
    #[strum(to_string="BR", serialize="BR")]
    VAL_BR,
    #[strum(to_string="EU", serialize="EU")]
    VAL_EU,
    #[strum(to_string="KR", serialize="KR")]
    VAL_KR,
    #[strum(to_string="LATAM", serialize="LATAM")]
    VAL_LATAM,
    #[strum(to_string="NA", serialize="NA")]
    VAL_NA,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("BR1", Region::BR.to_string());
    }

    #[test]
    fn test_get() {
        assert_eq!(Ok(Region::JP), "JP".parse());
        assert_eq!(Ok(Region::NA), "NA1".parse());
        assert!("LA".parse::<Region>().is_err());
    }
}