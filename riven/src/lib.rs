#![cfg_attr(not(target_family = "wasm"), doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", env!("CARGO_PKG_README"))))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Re-exported crates.
#[cfg(feature = "eserde")]
pub use eserde;
pub use ::{reqwest, serde, serde_json};

mod config;
pub use config::RiotApiConfig;
pub mod consts;
#[rustfmt::skip]
pub mod endpoints;
mod error;
pub use error::*;
pub mod meta;
#[cfg(feature = "metrics")]
mod metrics;
#[rustfmt::skip]
pub mod models;
mod models_impls;
mod req;
mod response_info;
pub use response_info::*;
mod riot_api;
pub use riot_api::*;
mod util;

/// Time items from either [`std::time`] or from [`web_time`] for wasm.
#[rustfmt::skip]
pub mod time {
    #[doc(no_inline)]
    pub use _time::{
        Duration,
        Instant,
        SystemTime,
        SystemTimeError,
        TryFromFloatSecsError,
    };
    mod _time {
        #[cfg(not(target_family = "wasm"))]
        pub use std::time::*;
        #[cfg(target_family = "wasm")]
        pub use web_time::*;
    }

    /// Re-exported [`tokio::time::sleep`], or [`gloo_timers::future::sleep`] for wasm.
    ///
    pub use _sleep::sleep;
    mod _sleep {
        #[cfg(not(target_family = "wasm"))]
        pub use tokio::time::sleep;
        #[cfg(target_family = "wasm")]
        pub use gloo_timers::future::sleep;
    }
}

/// JSON deserialization from [`serde`]/[`serde_json`], or [`eserde`] if the "eserde" feature is enabled.
#[rustfmt::skip]
pub mod de {
    /// [`serde::Deserialize`], or [`eserde::EDeserialize`] if the "eserde" feature is enabled.
    ///
    pub use _de::Deserialize;
    /// [`serde_json::Error`], or [`eserde::DeserializationErrors`] if the "eserde" feature is enabled.
    ///
    pub use _de::Error;
    /// [`serde_json::from_str`], or [`eserde::json::from_str`] if the "eserde" feature is enabled.
    ///
    pub use _de::from_str;
    /// [`serde_json::from_slice`], or [`eserde::json::from_slice`] if the "eserde" feature is enabled.
    ///
    pub use _de::from_slice;

    mod _de {
        #[cfg(not(feature = "eserde"))]
        pub use serde::Deserialize;
        #[cfg(not(feature = "eserde"))]
        pub use serde_json::{Error, from_str, from_slice};

        #[cfg(feature = "eserde")]
        pub use eserde::{Deserialize, EDeserialize as Deserialize, DeserializationErrors as Error};
        #[cfg(feature = "eserde")]
        pub use eserde::json::{from_str, from_slice};
    }
}
