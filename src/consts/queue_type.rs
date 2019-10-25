use std::fmt;

#[derive(fmt::Debug, Copy, Clone)]
pub enum QueueType {
    // League of Legends, Summoner's Rift (5v5), Ranked Solo Queue.
    RankedSolo5x5,
    // League of Legends, Summoner's Rift (5v5), Flex Queue.
    RankedFlexSr,
    // League of Legends, Twisted Treeline (3v3), Flex Queue.
    RankedFlexTt,
    // Ranked Teamfight Tactics.
    RankedTft,
}

impl QueueType {
    const NAMES: [&'static str; 4] = [
        "RANKED_SOLO_5x5",
        "RANKED_FLEX_SR",
        "RANKED_FLEX_TT",
        "RANKED_TFT",
    ];
}

impl fmt::Display for QueueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAMES[*self as usize])
    }
}
