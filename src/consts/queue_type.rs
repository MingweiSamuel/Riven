use strum_macros::{ EnumString, Display, AsRefStr };

#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, Display, AsRefStr)]
pub enum QueueType {
    // League of Legends, Summoner's Rift (5v5), Ranked Solo Queue.
    #[strum(to_string="RANKED_SOLO_5x5")]
    RankedSolo5x5,
    // League of Legends, Summoner's Rift (5v5), Flex Queue.
    #[strum(to_string="RANKED_FLEX_SR")]
    RankedFlexSr,
    // League of Legends, Twisted Treeline (3v3), Flex Queue.
    #[strum(to_string="RANKED_FLEX_TT")]
    RankedFlexTt,
    // Ranked Teamfight Tactics.
    #[strum(to_string="RANKED_TFT")]
    RankedTft,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_as_ref() {
        assert_eq!("RANKED_SOLO_5x5", QueueType::RankedSolo5x5.as_ref());
    }

    #[test]
    fn check_to_string() {
        assert_eq!("RANKED_SOLO_5x5", QueueType::RankedSolo5x5.to_string());
    }

    #[test]
    fn check_from_string() {
        assert_eq!(Some(QueueType::RankedSolo5x5), "RANKED_SOLO_5x5".parse().ok());
    }
}
