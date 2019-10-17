#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Region<'a> {
    pub key: &'a str,
    pub platform: &'a str,
}

macro_rules! regions {
    (
        $(
            $key:ident => $plat:expr ;
        )*
    ) => {
        $(
            pub const $key: &'static Region<'static> = &Region {
                key: stringify!($key),
                platform: $plat,
            };
        )*

        #[doc="Get region by name."]
        #[doc="# Arguments"]
        #[doc="* `name` - Case-insensitive ASCII string to match Regions' `key` or `playform`."]
        #[doc="# Returns"]
        #[doc="`Some(&Region)` if match found, `None` if no match found."]
        #[allow(unreachable_patterns)]
        pub fn get(name: &str) -> Option<&Region> {
            match &*name.to_ascii_uppercase() {
                $(
                    stringify!($key) | $plat => Some(Self::$key),
                )*
                _ => None
            }
        }
    }
}

impl Region<'_> {
    // Is this stupid?
    regions! {
        BR       => "BR1";
        EUNE     => "EUN1";
        EUW      => "EUW1";
        NA       => "NA1";
        KR       => "KR";
        LAN      => "LA1";
        LAS      => "LA2";
        OCE      => "OC1";
        RU       => "RU";
        TR       => "TR1";
        JP       => "JP1";
        PBE      => "PBE1";
        AMERICAS => "AMERICAS";
        EUROPE   => "EUROPE";
        ASIA     => "ASIA";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("BR1", Region::BR.platform);
    }

    #[test]
    fn test_get() {
        assert_eq!(Some(Region::AMERICAS), Region::get("amEricAs"));
        assert_eq!(Some(Region::NA), Region::get("na1"));
        assert_eq!(None, Region::get("LA"));
    }
}