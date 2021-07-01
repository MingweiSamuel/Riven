#![forbid(unsafe_code)]

#![doc = include_str!("../README.md")]

// Re-exported reqwest types.
pub use reqwest;


mod config;
pub use config::RiotApiConfig;

pub mod consts;

pub mod endpoints;

mod error;
pub use error::*;

pub mod meta;

pub mod models;

mod req;

mod response_info;
pub use response_info::*;

mod riot_api;
pub use riot_api::*;

mod util;
