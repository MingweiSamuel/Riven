#![cfg_attr(feature = "nightly", feature(non_exhaustive))]
#![cfg_attr(feature = "nightly", feature(external_doc))]

#![forbid(unsafe_code)]

#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(not(feature = "nightly"), doc = "See [README.md](https://github.com/MingweiSamuel/Riven#readme).")]

// Re-exported reqwest types.
pub use reqwest;


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
