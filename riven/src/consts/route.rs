use num_enum::{ IntoPrimitive, TryFromPrimitive };
use strum_macros::{ EnumString, EnumIter, Display, IntoStaticStr };

/// Regional routes, used in tournament services, Legends of Runterra, and other endpoints.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[derive(Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum RegionalRoute {
    /// Americas.
    AMERICAS = 1,

    /// Asia.
    ASIA = 2,

    /// Europe.
    EUROPE = 3,

    /// Southeast Asia. Only usable with the LoR endpoints.
    SEA = 4,
}

/// Platform routes for League of Legends and Teamfight Tactics.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[derive(Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
// Note: strum(serialize = ...) actuall specifies extra DEserialization values.
pub enum PlatformRoute {
    /// Brazil.
    #[strum(to_string="BR1", serialize="BR")]
    BR1 = 16,

    /// North-east Europe.
    #[strum(to_string="EUN1", serialize="EUNE")]
    EUN1 = 17,

    /// West Europe.
    #[strum(to_string="EUW1", serialize="EUW")]
    EUW1 = 18,

    /// Japan.
    #[strum(to_string="JP1", serialize="JP")]
    JP1 = 19,

    /// Korea.
    KR = 20,

    /// North Latin America.
    #[strum(to_string="LA1", serialize="LAN")]
    LA1 = 21,

    /// South Latin America.
    #[strum(to_string="LA2", serialize="LAS")]
    LA2 = 22,

    /// North America.
    #[strum(to_string="NA1", serialize="NA")]
    NA1 = 23,

    /// Oceania.
    #[strum(to_string="OC1", serialize="OCE")]
    OC1 = 24,

    /// Russia.
    RU = 25,

    /// Turkey.
    #[strum(to_string="TR1", serialize="TR")]
    TR1 = 26,


    /// Public beta environment. Only functional in certain endpoints.
    #[strum(to_string="PBE1", serialize="PBE")]
    PBE1 = 31,
}

impl PlatformRoute {
    pub fn to_regional(self) -> RegionalRoute {
        match self {
            Self::BR1  => RegionalRoute::AMERICAS,
            Self::LA1  => RegionalRoute::AMERICAS,
            Self::LA2  => RegionalRoute::AMERICAS,
            Self::NA1  => RegionalRoute::AMERICAS,
            Self::OC1  => RegionalRoute::AMERICAS,
            Self::PBE1 => RegionalRoute::AMERICAS,

            Self::JP1  => RegionalRoute::ASIA,
            Self::KR   => RegionalRoute::ASIA,

            Self::EUN1 => RegionalRoute::EUROPE,
            Self::EUW1 => RegionalRoute::EUROPE,
            Self::RU   => RegionalRoute::EUROPE,
            Self::TR1  => RegionalRoute::EUROPE,
        }
    }

    /// Used in Tournament API.
    pub fn as_region_str(self) -> &'static str {
        match self {
            Self::BR1  => "BR",
            Self::EUN1 => "EUNE",
            Self::EUW1 => "EUW",
            Self::JP1  => "JP",
            Self::LA1  => "LAN",
            Self::LA2  => "LAS",
            Self::NA1  => "NA",
            Self::OC1  => "OCE",
            Self::PBE1 => "PBE",
            Self::RU   => "RU",
            Self::TR1  => "TR",

            Self::KR   => "KR",
        }
    }
}

/// Platform routes for Valorant.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(EnumString, EnumIter, Display, IntoStaticStr)]
#[derive(Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum ValPlatformRoute {
    /// Valorant's Asian Pacific platform.
    AP = 64,

    /// Valorant's Brazil platform.
    BR = 65,

    /// Valorant's Europe platform.
    EU = 66,

    /// Valorant's Latin America platform.
    LATAM = 68,

    /// Valorant's North America platform.
    NA = 69,

    /// Valorant's Korea platform.
    KR = 70,


    /// Valorant's esports platform.
    ESPORTS = 95,
}

/// Utility enum containing all routing variants.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum Route {
    Regional(RegionalRoute),
    Platform(PlatformRoute),
    ValPlatform(ValPlatformRoute),
}

impl From<Route> for &'static str {
    fn from(route: Route) -> Self {
        match route {
            Route::Regional(r) => r.into(),
            Route::Platform(r) => r.into(),
            Route::ValPlatform(r) => r.into(),
        }
    }
}

impl From<Route> for u8 {
    fn from(route: Route) -> Self {
        match route {
            Route::Regional(r) => r.into(),
            Route::Platform(r) => r.into(),
            Route::ValPlatform(r) => r.into(),
        }
    }
}

impl num_enum::TryFromPrimitive for Route {
    type Primitive = u8;

    const NAME: &'static str = stringify!(Route);

    fn try_from_primitive(number: Self::Primitive) -> Result<Self, num_enum::TryFromPrimitiveError<Self>> {
        RegionalRoute::try_from_primitive(number)
            .map(|r| Route::Regional(r))
            .or_else(|_| PlatformRoute::try_from_primitive(number)
                .map(|r| Route::Platform(r)))
            .or_else(|_| ValPlatformRoute::try_from_primitive(number)
                .map(|r| Route::ValPlatform(r)))
            .map_err(|_| num_enum::TryFromPrimitiveError { number })
    }
}

impl std::convert::TryFrom<u8> for Route {
    type Error = num_enum::TryFromPrimitiveError<Self>;
    fn try_from(number: u8) -> Result<Self, num_enum::TryFromPrimitiveError<Self>> {
        Self::try_from_primitive(number)
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regional(r) => r.fmt(f),
            Self::Platform(r) => r.fmt(f),
            Self::ValPlatform(r) => r.fmt(f),
        }
    }
}

impl std::str::FromStr for Route {
    type Err = strum::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RegionalRoute::from_str(s)
            .map(|r| Self::Regional(r))
            .or_else(|_| PlatformRoute::from_str(s)
                .map(|r| Self::Platform(r)))
            .or_else(|_| ValPlatformRoute::from_str(s)
                .map(|r| Self::ValPlatform(r)))
            .map_err(|_| strum::ParseError::VariantNotFound)
    }
}

impl Route {
    pub fn iter() -> impl Iterator<Item = Self> {
        use strum::IntoEnumIterator;

        let regional = RegionalRoute::iter()
            .map(|r| Self::Regional(r));
        let platform = PlatformRoute::iter()
            .map(|r| Self::Platform(r));
        let val_platform = ValPlatformRoute::iter()
            .map(|r| Self::ValPlatform(r));

        regional
            .chain(platform)
            .chain(val_platform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_tostring() {
        assert_eq!("AMERICAS", Into::<&'static str>::into(Route::Regional(RegionalRoute::AMERICAS)));
        assert_eq!("KR", Into::<&'static str>::into(Route::Platform(PlatformRoute::KR)));
        assert_eq!("KR", Into::<&'static str>::into(Route::ValPlatform(ValPlatformRoute::KR)));
    }

    #[test]
    fn test_route_iter() {
        for (i, route) in Route::iter().enumerate() {
            println!("{:>2}   {:<10}  {:>3}", i, route, u8::from(route));
        }
    }

    #[test]
    fn test_route_tryfrom() {
        for x in u8::MIN..=u8::MAX {
            if let Ok(route) = std::convert::TryInto::<Route>::try_into(x) {
                println!("{:>3}   {:<8}", x, route);
            }
        }
    }

    #[test]
    fn test_regional_tostring() {
        assert_eq!("AMERICAS", RegionalRoute::AMERICAS.to_string());
        assert_eq!("SEA", RegionalRoute::SEA.to_string());

        assert_eq!("AMERICAS", Into::<&'static str>::into(RegionalRoute::AMERICAS));
        assert_eq!("SEA", Into::<&'static str>::into(RegionalRoute::SEA));
    }

    #[test]
    fn test_regional_parse() {
        assert_eq!(Ok(RegionalRoute::AMERICAS), "AMERICAS".parse());
        assert_eq!(Ok(RegionalRoute::SEA), "SEA".parse());
        assert!("NA".parse::<RegionalRoute>().is_err());
    }

    #[test]
    fn test_platform_tostring() {
        assert_eq!("BR1", PlatformRoute::BR1.to_string());
        assert_eq!("KR", PlatformRoute::KR.to_string());

        assert_eq!("BR1", Into::<&'static str>::into(PlatformRoute::BR1));
        assert_eq!("KR", Into::<&'static str>::into(PlatformRoute::KR));
    }

    #[test]
    fn test_platform_parse() {
        assert_eq!(Ok(PlatformRoute::BR1), "BR1".parse());
        assert_eq!(Ok(PlatformRoute::KR), "KR".parse());
        assert_eq!(Ok(PlatformRoute::JP1), "JP1".parse());
        assert_eq!(Ok(PlatformRoute::JP1), "JP".parse());
        assert_eq!(Ok(PlatformRoute::NA1), "NA1".parse());
        assert_eq!(Ok(PlatformRoute::NA1), "NA".parse());
        assert!("LA".parse::<PlatformRoute>().is_err());
    }

    #[test]
    fn test_valplatform_tostring() {
        assert_eq!("AP", ValPlatformRoute::AP.to_string());
        assert_eq!("KR", ValPlatformRoute::KR.to_string());
        assert_eq!("ESPORTS", ValPlatformRoute::ESPORTS.to_string());

        assert_eq!("AP", Into::<&'static str>::into(ValPlatformRoute::AP));
        assert_eq!("KR", Into::<&'static str>::into(ValPlatformRoute::KR));
        assert_eq!("ESPORTS", Into::<&'static str>::into(ValPlatformRoute::ESPORTS));
    }

    #[test]
    fn test_valplatform_parse() {
        assert_eq!(Ok(ValPlatformRoute::AP), "AP".parse());
        assert_eq!(Ok(ValPlatformRoute::KR), "KR".parse());
        assert_eq!(Ok(ValPlatformRoute::ESPORTS), "ESPORTS".parse());
        assert!("SEA".parse::<ValPlatformRoute>().is_err());
    }
}
