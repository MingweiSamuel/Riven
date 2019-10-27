//! Constant data and Enums relevant to the Riot Games API.
//!
//! This module uses SCREAMING_SNAKE_CASE for enum variants, as enums in this
//! crate should be considered collections of constants.

#![allow(deprecated)]
#![allow(non_camel_case_types)]

mod macro_serde_string;

mod champion;
pub use champion::*;

mod division;
pub use division::*;

mod game_mode;
pub use game_mode::*;

mod game_type;
pub use game_type::*;

mod map;
pub use map::*;

mod queue_type;
pub use queue_type::*;

mod queue;
pub use queue::*;

mod region;
pub use region::*;

mod season;
pub use season::*;

mod team;
pub use team::*;

mod tier;
pub use tier::*;
