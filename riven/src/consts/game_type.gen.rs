// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version: 996d171a2b79e9bb85c549f47b07c6ef2721fc8a

use strum_macros::{EnumString, Display, AsRefStr, IntoStaticStr};
/// League of Legends game type: matched game, custom game, or tutorial game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
#[derive(serde::Serialize, crate::de::Deserialize)]
#[repr(u8)]
pub enum GameType {
    ///Custom games
    #[strum(to_string = "CUSTOM_GAME", serialize = "CUSTOM")]
    #[serde(alias = "CUSTOM")]
    CUSTOM_GAME,
    ///all other games
    #[strum(to_string = "MATCHED_GAME", serialize = "MATCHED")]
    #[serde(alias = "MATCHED")]
    MATCHED_GAME,
    ///Tutorial games
    #[strum(to_string = "TUTORIAL_GAME", serialize = "TUTORIAL")]
    #[serde(alias = "TUTORIAL")]
    TUTORIAL_GAME,
}

