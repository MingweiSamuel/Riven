use strum_macros::{ EnumString, EnumVariantNames, IntoStaticStr };

/// LoL or TFT ranked queue types.
#[non_exhaustive]
#[derive(Debug, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, EnumVariantNames, IntoStaticStr)]
pub enum QueueType {
    /// Catch-all variant for new, unknown queue types.
    #[strum(default)]
    UNKNOWN(String),

    /// League of Legends, Summoner's Rift (5v5), Ranked Solo Queue.
    RANKED_SOLO_5x5,
    /// League of Legends, Summoner's Rift (5v5), Flex Queue.
    RANKED_FLEX_SR,
    /// League of Legends, Twisted Treeline (3v3), Flex Queue.
    RANKED_FLEX_TT,
    /// Ranked Teamfight Tactics.
    RANKED_TFT,
    /// Ranked Teamfight Tactics, Hyper Roll gamemode.
    RANKED_TFT_TURBO,
    /// Ranked Teamfight Tactics, Double Up gamemode.
    RANKED_TFT_DOUBLE_UP,

    /// Ranked Teamfight Tactics, OLD Double Up gamemode. Changed some time before June 2022.
    #[deprecated(note="Use RANKED_TFT_DOUBLE_UP instead.")]
    RANKED_TFT_PAIRS,
}

serde_strum_unknown!(QueueType);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_as_ref() {
        assert_eq!("RANKED_SOLO_5x5", QueueType::RANKED_SOLO_5x5.as_ref());
    }

    #[test]
    fn check_to_string() {
        assert_eq!("RANKED_SOLO_5x5", QueueType::RANKED_SOLO_5x5.to_string());
    }

    #[test]
    fn check_from_string() {
        assert_eq!(QueueType::RANKED_SOLO_5x5, "RANKED_SOLO_5x5".into());
        assert_eq!(QueueType::UNKNOWN("RANKED_MYSTERY_UNKNOWN".to_owned()), "RANKED_MYSTERY_UNKNOWN".into());
        assert_eq!("RANKED_MYSTERY_UNKNOWN", QueueType::UNKNOWN("RANKED_MYSTERY_UNKNOWN".to_owned()).as_ref());
    }

    #[test]
    fn check_serialize() {
        assert_eq!(Some("\"RANKED_TFT_DOUBLE_UP\""),
            serde_json::to_string(&QueueType::RANKED_TFT_DOUBLE_UP)
                .ok().as_deref());
        assert_eq!(Some("\"RANKED_MYSTERY_UNKNOWN\""),
            serde_json::to_string(&QueueType::UNKNOWN("RANKED_MYSTERY_UNKNOWN".to_owned()))
                .ok().as_deref());
    }

    #[test]
    // Note: this test is often not run due to this condition below.
    #[cfg(not(feature = "deny-unknown-enum-variants-strings"))]
    fn check_deserialize() {
        use std::collections::BTreeMap;

        let dict: BTreeMap<usize, QueueType> = serde_json::from_str(
            r#"{
                "100": "RANKED_SOLO_5x5",
                "200": "RANKED_TFT_TURBO",
                "210": "RANKED_TFT_DOUBLE_UP",
                "211": "RANKED_TFT_PAIRS",
                "900": "RANKED_MYSTERY_UNKNOWN"
            }"#
        ).unwrap();

        assert_eq!(Some(&QueueType::RANKED_SOLO_5x5), dict.get(&100));
        assert_eq!(Some(&QueueType::RANKED_TFT_TURBO), dict.get(&200));
        assert_eq!(Some(&QueueType::RANKED_TFT_DOUBLE_UP), dict.get(&210));
        assert_eq!(Some(&QueueType::RANKED_TFT_PAIRS), dict.get(&211));
        assert_eq!(Some(&QueueType::UNKNOWN("RANKED_MYSTERY_UNKNOWN".to_owned())), dict.get(&900));

    }
}
