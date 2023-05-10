use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// League of Legends team.
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize_repr,
    Deserialize_repr,
    IntoPrimitive,
    TryFromPrimitive,
)]
#[repr(u16)]
pub enum Team {
    /// Blue team (bottom left on Summoner's Rift).
    BLUE = 100,
    /// Red team (top right on Summoner's Rift).
    RED = 200,

    /// "killerTeamId" when Baron Nashor spawns and kills Rift Herald.
    OTHER = 300,
}
