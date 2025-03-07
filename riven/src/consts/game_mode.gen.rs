// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version: 996d171a2b79e9bb85c549f47b07c6ef2721fc8a

use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr};
///League of Legends game mode, such as Classic, ARAM, URF, One For All, Ascension, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(EnumString, EnumVariantNames, IntoStaticStr)]
#[repr(u8)]
pub enum GameMode {
    /// Catch-all variant for new/unknown values.
    #[strum(default)]
    UNKNOWN(String),
    ///ARAM games
    ARAM,
    ///All Random Summoner's Rift games
    ARSR,
    ///Ascension games
    ASCENSION,
    ///Blood Hunt Assassin games
    ASSASSINATE,
    ///2v2v2v2 Arena
    CHERRY,
    ///Classic Summoner's Rift and Twisted Treeline games
    CLASSIC,
    ///Dark Star: Singularity games
    DARKSTAR,
    ///Doom Bot games
    DOOMBOTSTEEMO,
    ///Snowdown Showdown games
    FIRSTBLOOD,
    ///Nexus Blitz games
    GAMEMODEX,
    ///Legend of the Poro King games
    KINGPORO,
    ///Nexus Blitz games
    NEXUSBLITZ,
    ///Dominion/Crystal Scar games
    ODIN,
    ///Odyssey: Extraction games
    ODYSSEY,
    ///One for All games
    ONEFORALL,
    ///Practice tool training games.
    PRACTICETOOL,
    ///PROJECT: Hunters games
    PROJECT,
    ///Nexus Siege games
    SIEGE,
    ///Star Guardian Invasion games
    STARGUARDIAN,
    ///Swarm
    STRAWBERRY,
    ///Swiftplay Summoner's Rift
    SWIFTPLAY,
    ///Teamfight Tactics.
    TFT,
    ///Tutorial games
    TUTORIAL,
    ///Tutorial: Welcome to League.
    TUTORIAL_MODULE_1,
    ///Tutorial: Power Up.
    TUTORIAL_MODULE_2,
    ///Tutorial: Shop for Gear.
    TUTORIAL_MODULE_3,
    ///Ultimate Spellbook games
    ULTBOOK,
    ///URF games
    URF,
}
serde_strum_unknown!(GameMode);

