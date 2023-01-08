use super::{ RegionalRoute, PlatformRoute, ValPlatformRoute };

/// Utility enum containing all routing variants.
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum Route {
    /// Sub-variant for [`RegionalRoute`]s.
    Regional(RegionalRoute),
    /// Sub-variant for [`PlatformRoute`]s.
    Platform(PlatformRoute),
    /// Sub-variant for [`ValPlatformRoute`]s.
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
            .map(Route::Regional)
            .or_else(|_| PlatformRoute::try_from_primitive(number)
                .map(Route::Platform))
            .or_else(|_| ValPlatformRoute::try_from_primitive(number)
                .map(Route::ValPlatform))
            .map_err(|_| num_enum::TryFromPrimitiveError { number })
    }
}

impl std::convert::TryFrom<u8> for Route {
    type Error = num_enum::TryFromPrimitiveError<Self>;
    fn try_from(number: u8) -> Result<Self, num_enum::TryFromPrimitiveError<Self>> {
        <Self as num_enum::TryFromPrimitive>::try_from_primitive(number)
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
            .map(Self::Regional)
            .or_else(|_| PlatformRoute::from_str(s)
                .map(Self::Platform))
            .or_else(|_| ValPlatformRoute::from_str(s)
                .map(Self::ValPlatform))
            .map_err(|_| strum::ParseError::VariantNotFound)
    }
}

impl Route {
    /// Returns an iterator over all routes. Starts with [`Self::Regional`],
    /// then [`Self::Platform`], and finally [`Self::ValPlatform`].
    pub fn iter() -> impl Iterator<Item = Self> {
        use strum::IntoEnumIterator;

        let regional = RegionalRoute::iter()
            .map(Self::Regional);
        let platform = PlatformRoute::iter()
            .map(Self::Platform);
        let val_platform = ValPlatformRoute::iter()
            .map(Self::ValPlatform);

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
