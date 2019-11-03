#![feature(non_exhaustive)]
#![feature(external_doc)]

#![forbid(unsafe_code)]

#![doc(include = "../README.md")]

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
