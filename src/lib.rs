//! Module docs TODO.
#![feature(non_exhaustive)]

mod config;
pub use config::RiotApiConfig;

pub mod consts;

pub mod endpoints;

mod error;
pub use error::*;

pub mod models;

mod req;

mod riot_api;
pub use riot_api::*;

mod util;
