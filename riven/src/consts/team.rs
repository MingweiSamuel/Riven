use serde_repr::{ Serialize_repr, Deserialize_repr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// League of Legends team.
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Team {
    /// Blue team (bottom left on Summoner's Rift).
    BLUE = 100,
    /// Red team (top right on Summoner's Rift).
    RED = 200,

    /// "killerTeamId" when Baron Nashor spawns and kills Rift Herald.
    OTHER = 300,
}
