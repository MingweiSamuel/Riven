use serde::{ Serialize, Deserialize };
use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };

/// LoL or TFT ranked queue types.
#[non_exhaustive]
#[derive(Debug, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
#[derive(Serialize, Deserialize)]
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
        assert_eq!(Some(QueueType::RANKED_SOLO_5x5), "RANKED_SOLO_5x5".parse().ok());
    }
}
