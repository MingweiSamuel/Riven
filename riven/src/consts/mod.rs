//! Constant data and Enums used with the Riot Games API.
//!
//! This module uses SCREAMING_SNAKE_CASE for enum variants, as enums in this
//! crate should be considered collections of constants.

#![allow(deprecated)]
#![allow(non_camel_case_types)]

/// Trait allowing iteration of enum types, implemented by several enums in this module.
/// Re-exported from strum.
pub use strum::IntoEnumIterator;

mod champion;
pub use champion::*;

mod division;
pub use division::*;

mod game_mode {
    include_autogen!("game_mode.gen.rs");
}
pub use game_mode::*;

mod game_type;
pub use game_type::*;

mod map {
    include_autogen!("map.gen.rs");
}
pub use map::*;

mod queue_type;
pub use queue_type::*;

mod queue {
    include_autogen!("queue.gen.rs");
}
pub use queue::*;

pub mod ranks;

mod route;
pub use route::*;

mod season {
    include_autogen!("season.gen.rs");
}
pub use season::*;

mod team;
pub use team::*;

mod tier;
pub use tier::*;

/// https://github.com/RiotGames/developer-relations/issues/898
pub(crate) fn serialize_empty_string_none<S, T>(
    val: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: serde::ser::Serialize,
{
    use serde::ser::Serialize;
    if let Some(val) = val {
        val.serialize(serializer)
    } else {
        "".serialize(serializer)
    }
}

/// https://github.com/RiotGames/developer-relations/issues/898
pub(crate) fn deserialize_empty_string_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: serde::de::Deserialize<'de>,
{
    use serde::de::{Deserialize, IntoDeserializer};
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}
