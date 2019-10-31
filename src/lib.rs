//! Module docs TODO.
#![feature(non_exhaustive)]

mod riot_api_error;
pub use riot_api_error::*;

pub mod consts;

pub mod endpoints;

pub mod riot_api_config;
pub use riot_api_config::RiotApiConfig;

mod riot_api;
pub use riot_api::*;

mod req;
mod util;
