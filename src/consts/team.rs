#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd)]
/// League of Legends team.
pub enum Team {
    /// Blue team (bottom left on Summoner's Rift).
    Blue = 100,
    /// Red team (top right on Summoner's Rift).
    Red = 200,
}
