//! Constant data and Enums used with the Riot Games API.
//!
//! This module uses SCREAMING_SNAKE_CASE for enum variants, as enums in this
//! crate should be considered collections of constants.

#![allow(deprecated)]
#![allow(non_camel_case_types)]

mod macros;

#[rustfmt::skip]
mod champion;
pub use champion::*;

mod division;
pub use division::*;

#[rustfmt::skip]
mod game_mode;
pub use game_mode::*;

#[rustfmt::skip]
mod game_type;
pub use game_type::*;

#[rustfmt::skip]
mod map;
pub use map::*;

#[rustfmt::skip]
mod queue_type;
pub use queue_type::*;

#[rustfmt::skip]
mod queue;
pub use queue::*;

pub mod ranks;

#[rustfmt::skip]
mod route;
pub use route::*;

mod route_ext;
pub use route_ext::*;

#[rustfmt::skip]
mod season;
pub use season::*;

/// Trait allowing iteration of enum types, implemented by several enums in this module.
/// Re-exported from strum.
///
///
pub use strum::IntoEnumIterator;

mod team;
pub use team::*;

mod tier;
pub use tier::*;
